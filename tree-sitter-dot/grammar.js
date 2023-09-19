const keyword_strict = 'strict'
const keyword_graph = 'graph'
const keyword_digraph = 'digraph'
const keyword_subgraph = 'subgraph'

module.exports = grammar({
    name: 'dot',

    extras: $ => [
        /\s|\r?\n/,
        $.comment,
        $.preproc,
    ],

    word: $ => $.name,

    rules: {
        source_file: $ => seq(
            field('strict', optional(alias(ignoreCase(keyword_strict), keyword_strict))),
            field('graph_or_digraph', $.graph_or_digraph),
            field('id', optional($.id)),
            field('block', $.block),
        ),



        graph_or_digraph: $ => choice(
            field('graph', alias(ignoreCase(keyword_graph), keyword_graph)),
            field('digraph', alias(ignoreCase(keyword_digraph), keyword_digraph)),
        ),

        block: $ => seq(
            field('left_brackets', '{'),
            field('stmt_list', optional($.stmt_list)),
            field('right_brackets', '}'),
        ),

        stmt_list: $ => repeat1(
            seq(
                field("stmt", $.stmt),
                alias(optional(';'), 'semicolon'),
            )
        ),

        stmt: $ => choice(
            field('subgraph', $.subgraph),
            field('node_stmt', $.node_stmt),
            field('edge_stmt', $.edge_stmt),
            field('attr_stmt', $.attr_stmt),
            field('attribute', $.attribute),
        ),

        keywords: $=> choice(
            keyword_strict,
            keyword_graph,
            keyword_digraph,
            keyword_subgraph
        ),

        subgraph: $ => seq(
            field("subgraph_id",
                optional(
                    seq(
                        field('subgraph', alias(ignoreCase(keyword_subgraph), keyword_subgraph)),
                        optional(field('id', $.id)),
                    ),
                )
            ),
            field('block', $.block),
        ),

        node_stmt: $ => seq(
            field('node_id', $.node_id),
            optional(field('attr_list', $.attr_list)),
        ),

        edge_stmt: $ => seq(
            choice(
                field('left_node_id', $.node_id),
                field('left_subgraph', $.subgraph),
            ),
            repeat1(
                seq(
                    field('edgeop', $.edgeop),
                    choice(
                        field('right_node_id', $.node_id),
                        field('right_subgraph', $.subgraph),
                    )
                )
            ),
            optional(field('attr_list', $.attr_list)),
        ),

        attr_stmt: $ => seq(
            field('attr_type',
                $.attr_type
            ),
            field('attr_list', $.attr_list),
        ),

        attr_type: $=>  choice(
            field('graph', alias(ignoreCase('graph'), '')),
            field('node', alias(ignoreCase('node'), 'node')),
            field('edge', alias(ignoreCase('edge'), 'edge')),
        ),

        node_id: $ => seq(
            field('id', $.id),
            optional(field('port', $.port)),
        ),

        port: $ => seq(
            field('operator', alias(':', $.operator)),
            field('id', $.id),
            field('optional',
                optional(
                    seq(
                        field('opt_operator', alias(':', $.operator)),
                        field('opt_id', $.id),
                    )
                )
            ),
        ),

        attr_list: $ => repeat1(
            seq(
                field('left_bracket', '['),
                optional($._a_list),
                field('right_bracket', ']'),
            ),
        ),

        _a_list: $ => repeat1(seq(
            field('attribute', $.attribute),
            optional(field('semicolon', choice(';', ','))),
        )),

        attribute: $ => seq(
            field('name', $.id),
            alias('=', $.operator),
            field('value', $.id),
        ),

        id: $ => choice(
            field('identifier', $.identifier),
            field('number_literal', $.number_literal),
            field('html_string', $.html_string),

            seq(
                field('string_literal', $.string_literal),
                repeat(seq(
                    alias('+', $.operator),
                    field('string_literal', $.string_literal),
                )),
            )
        ),

        name: $=> /[_\p{XID_Start}][_\p{XID_Continue}]*/,

        identifier: $ => choice(
            $.keywords,
            $.name
        ),

        number_literal: _ => /-?(\.[0-9]+|[0-9]+(\.[0-9]*)?)/,

        string_literal: _ => seq(
            '"',
            /([^"\\]+|\\.)*"/,
        ),

        html_string: $ => seq(
            '<',
            optional(alias($._html_internal, $.html_internal)),
            '>',
        ),

        _html_internal: $ => repeat1(
            choice(
                /[^<>]+/,
                seq(
                    '<',
                    optional($._html_internal),
                    '>',
                ),
            ),
        ),

        edgeop: _ => choice(
            field("arrow", '->'),
            field('line', '--')
        ),

        comment: _ => token(
            choice(
                seq('//', /.*/),
                seq(
                    '/*',
                    /[^*]*\*+([^/*][^*]*\*+)*/,
                    '/',
                ),
            ),
        ),

        preproc: _ => seq('#', /.*/),
    }
});

function ignoreCase(word) {
    return new RegExp(
        word.split('')
            .map(char => `[${char}${char.toUpperCase()}]`)
            .join(''))
}

// A sequence of one or more comma-separated strings matching the rule
function commaList1(rule) {
    return seq(rule, repeat(seq(',', rule)))
}

// A sequence of zero or more comma-separated strings matching the rule
function commaList(rule) {
    return optional(commaList1(rule))
}