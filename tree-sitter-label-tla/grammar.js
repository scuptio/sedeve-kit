// TLA+ expression parser
// reference from:
// https://github.com/tlaplus-community/tree-sitter-tlaplus/blob/main/grammar.js

module.exports = grammar({
    name: 'label_tla',

    rules: {
        tla_state_conjunction_variable_value_list: $ => repeat1(
            $.tla_conjunction_variable_value,
        ),

        tla_conjunction_variable_value: $ => seq(
            field('conjunction', $.tla_conjunction),
            field('tla_variable_value', $.tla_variable_value),
        ),

        tla_variable_value: $ => seq(
            field('tla_variable_value_left', $.tla_variable),
            '=',
            field('tla_variable_value_right', $.tla_value_expr),
        ),

        tla_action_name: _ => /[a-zA-Z_]+[a-zA-Z_0-9]+/,

        // [foo |-> 0, bar |-> 1]
        tla_record: $ => seq(
            '[',
            field("tla_record_item_comma_list", optional($.tla_record_item_comma_list)),
            ']'
        ),

        tla_record_item_comma_list : $=> commaList1($.tla_record_field),

        tla_record_field: ($) => seq(
            field("tla_record_field_left", $.tla_variable),
            field("tla_record_variable_map_to_to", $.tla_all_map_to),
            field("tla_record_field_right", $.tla_value_expr)
        ),

        tla_value_expr: $ => choice(
            field("tla_number", $.tla_number),
            field("tla_string", $.tla_string),
            field("tla_record", $.tla_record),
            field("tla_sequence", $.tla_sequence),
            field("tla_finite_set", $.tla_finite_set),
            field("tla_function", $.tla_function),
            field("tla_variable", $.tla_variable),
            field("tla_boolean", $.tla_boolean)
        ),

        // (1 :> "xxx")
        tla_function: $ => seq(
            '(',
            field("tla_function_comma_list", optional($.tla_function_comma_list)
            ),
            ')'
        ),

        tla_function_comma_list: $=> atatList1(
            $.tla_mapping
        ),

        tla_mapping: ($) => seq(
            field("tla_record_field_left", $.tla_value_expr),
            field("tla_map_to", $.tla_map_to),
            field("tla_record_field_right", $.tla_value_expr)
        ),

        // {1, 2, 3, 4, 5}
        tla_finite_set: $ => seq(
            '{',
            field("tla_finite_set_item_comma_list", optional($.tla_finite_set_item_comma_list)),
            '}'
        ),

        tla_finite_set_item_comma_list: $ => commaList1($.tla_value_expr),

        // <<1,2,3,4,5>>, <<>>
        tla_sequence: $ => seq(
            $.tla_langle_bracket,
            field("tla_sequence_item_comma_list", optional($.tla_sequence_item_comma_list)),
            $.tla_rangle_bracket
        ),

        tla_sequence_item_comma_list: $ => commaList1($.tla_value_expr),
        // tla_value_expr_list: $ => commaList($.tla_value_expr),

        // "foobar", "", etc.
        tla_string: $ => seq(
            '"',
            repeat(choice(
                token.immediate(/[^"\n]/),
                $.tla_escape_char
            )),
            token.immediate('"')
        ),

        tla_boolean: $ => choice(
            'TRUE',
            'FALSE'
        ),

        // "/\\", "say \"hello\" back", "one\ntwo"
        tla_escape_char: _ => seq(
            token.immediate('\\'),
            token.immediate(/./)
        ),

        tla_number: $ => choice(
            field("tla_nat_number", $.tla_nat_number),
            field("tla_real_number", $.tla_real_number),
            field("tla_binary_number", $.tla_binary_number),
            field("tla_octal_number", $.tla_octal_number),
            field("tla_hex_number", $.tla_hex_number)
        ),

        // 12345
        tla_nat_number: _ => /\d+/,

        // 3.14159
        tla_real_number: _ => /\d*\.\d+/,

        // \B01010101
        tla_binary_number: $ => seq(
            alias(choice('\\b', '\\B'), $.format),
            alias(token.immediate(/[0-1]+/), $.value)
        ),

        // \O01234567
        tla_octal_number: $ => seq(
            alias(choice('\\o', '\\O'), $.format),
            alias(token.immediate(/[0-7]+/), $.value)
        ),

        // \HDEADC0DE
        tla_hex_number: $ => seq(
            alias(choice('\\h', '\\H'), $.format),
            alias(token.immediate(/[0-9a-fA-F]+/), $.value)
        ),

        tla_all_map_to: _ => '|->',
        tla_langle_bracket: _ => '<<',
        tla_rangle_bracket: _ => '>>',
        tla_map_to: _ => ':>',
        tla_variable: _ => /[0-9_]*[A-Za-z][A-Za-z0-9_]*/,
        tla_conjunction: _ => '/\\',
    }
});

function atatList1(rule) {
    return seq(rule, repeat(seq('@@', rule)))
}

// A sequence of one or more comma-separated strings matching the rule
function commaList1(rule) {
    return seq(rule, repeat(seq(',', rule)))
}

// A sequence of zero or more comma-separated strings matching the rule
function commaList(rule) {
    return optional(commaList1(rule))
}