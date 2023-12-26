// action definition parser



// keyword definition

const control_automaton = 'automata'
const control_struct = 'struct'
const control_enum = 'enum'
const control_message = 'message'
const control_unknown = 'unknown'
const control_from = 'from'
const control_to = 'to'

const internal_type_node_id = 'NodeId'
const internal_type_set = 'Set'
const internal_type_sequence = 'Sequence'
const internal_type_message = "MsgTrait"

const primitive_type_bool   =       'bool'
const primitive_type_i8     =       'i8'
const primitive_type_i16    =       'i16'
const primitive_type_i32    =       'i32'
const primitive_type_i64    =       'i64'
const primitive_type_u8     =       'u8'
const primitive_type_u16    =       'u16'
const primitive_type_u32    =       'u32'
const primitive_type_u64    =       'u64'
const primitive_type_f32    =       'f32'
const primitive_type_f64    =       'f64'
const primitive_type_string =       'string'

const action_input      = 'input'
const action_output     = 'output'
const action_internal   = 'internal'

function comma_separate1 (rule) {
    return seq(rule, repeat(seq(',', rule)), optional(','))
}


module.exports = grammar({
    name: 'action',


    rules: {
        //action_file: $=> $.action_definition,

        source_file: $=> repeat1(
            $.automaton_or_type_definition
        ),

        keywords: $ => choice(
            $.keyword_primitive_type,
            $.keyword_internal_type,
            $.keyword_control,
            $.keyword_action,
        ),

        keyword_action: $=> choice(
            action_input,
            action_output,
            action_internal,
        ),

        keyword_control: $=> choice(
            control_struct,
            control_enum,
            control_unknown,
            control_message,
            control_from,
            control_to,
            control_automaton
        ),

        keyword_internal_type: $=> choice(
            internal_type_sequence,
            internal_type_set,
            internal_type_node_id,
        ),

        keyword_primitive_type: $ => choice(
            primitive_type_bool,
            primitive_type_i8,
            primitive_type_i16,
            primitive_type_i32,
            primitive_type_i64,
            primitive_type_u8,
            primitive_type_u16,
            primitive_type_u32,
            primitive_type_u64,
            primitive_type_f32,
            primitive_type_f64,
            primitive_type_string,
        ),


        automaton_or_type_definition: $ => choice(
            field('automaton_definition', $.automaton_definition)
            ,
            field('struct_definition', $.struct_definition)
            ,
            field('enum_definition', $.enum_definition),
        ),

        struct_definition: $=> seq(
          control_struct,
          field('struct_name', $.struct_name),
          '{',
          field('struct_field_list', $.struct_field_list),
          '}',
        ),

        enum_definition: $=> seq(
            control_enum,
            field('enum_name', $.enum_name),
            '{',
            field('enum_value_definition_list', $.enum_value_definition_list),
            '}'
        ),

        struct_field_list: $=> comma_separate1($.struct_field),

        struct_field: $=> seq(
            field('field_name', $.field_name),
            ':',
            field('data_type', $.data_type),
        ),


        enum_value_definition_list: $=> comma_separate1($.enum_value_definition),

        enum_value_definition: $=> seq(
            field('enum_value_name', $.enum_value_name),
            optional(field('enum_payload', $.enum_payload)),
        ),

        enum_payload: $=> seq(
            '(',
            field('data_type',$.data_type),
            ')'
        ),

        data_type: $=> choice(
            field('primitive_type', $.keyword_primitive_type),
            field('user_define_type', $.user_define_type),
            field('internal_type', $.internal_type)
        ),

        user_define_type: $=> seq(
            optional(field('data_type_unknown', control_unknown)),
            field('data_type_name', $.type_name),
        ),

        internal_type: $=> choice(
            field('data_type_node_id', internal_type_node_id),
            field('data_type_sequence', $.sequence_type),
            field('data_type_set', $.set_type),
            field("data_type_message", $.message_type)
        ),

        sequence_type: $=> seq(
            internal_type_sequence,
            '<',
            field('data_type', $.data_type),
            '>'
        ),

        set_type: $=> seq(
            internal_type_set,
            '<',
                field('data_type', $.data_type),
            '>'
        ),

        message_type: $=> seq(
            internal_type_message,
            '<',
                field('data_type', $.data_type),
            '>'
        ),

        automaton_definition: $=> seq(
            control_automaton,
            field('automation_name', $.automation_name),
            '{',
            field('action_definition_list', $.action_definition_list),
            '}'
        ),

        action_definition_list: $ => repeat1(
            $.action_definition
        ),

        action_type: $=> choice(
            action_input,
            action_output,
            action_internal,
        ),

        action_definition: $ => seq(
            field('action_type', $.action_type),
            field('action_handle_name', $.action_handle_name),
            '(',
            field('parameter_list', $.parameter_list),
            ')',
            optional(field('return_value', $.handle_return))
        ),

        handle_return: $=> seq(
            '->',
            field('data_type' , $.data_type)
        ),

        parameter_list: $ => seq(
            field('message_parameter', $.message_parameter),
            optional(
                seq(
                    ',',
                    field('from_parameter', $.from_parameter),
                    field('to_parameter', $.to_parameter)
                )
            ),
            optional(seq(',' , field('other_parameter_list', $.other_parameter_list))),
        ),


        message_parameter: $ => seq(
            field('parameter_name', control_message),
            ':',
            field('user_define_type', $.user_define_type)
        ),

        from_parameter: $ => seq(
            field('parameter_name', control_from),
            ':',
            field('type_node_id', internal_type_node_id)
        ),

        to_parameter: $ => seq(
            field('parameter_name', control_to),
            ':',
            field('type_node_id', internal_type_node_id),
        ),

        other_parameter_list: $ => comma_separate1(
            $.parameter,
        ),

        parameter: $ => seq(
            field('parameter_name', $.parameter_name),
            ':',
            field('parameter_type', $.data_type),
        ),

        struct_name: $=> $.identifier,
        enum_name: $=> $.identifier,
        enum_value_name: $=> $.identifier,
        parameter_name: $=> $.identifier,
        action_handle_name: $ => $.identifier,
        field_name: $ => $.identifier,
        automation_name: $ => $.identifier,
        type_name: $ => $.identifier,

        identifier: $ => /[a-zA-Z_]\w*/,
    }
});

