#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 76
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 61
#define ALIAS_COUNT 0
#define TOKEN_COUNT 35
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 25
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 22

enum ts_symbol_identifiers {
  anon_sym_EQ = 1,
  sym_tla_action_name = 2,
  anon_sym_LBRACK = 3,
  anon_sym_RBRACK = 4,
  anon_sym_COMMA = 5,
  anon_sym_LPAREN = 6,
  anon_sym_RPAREN = 7,
  anon_sym_AT_AT = 8,
  anon_sym_LBRACE = 9,
  anon_sym_RBRACE = 10,
  anon_sym_DQUOTE = 11,
  aux_sym_tla_string_token1 = 12,
  anon_sym_DQUOTE2 = 13,
  anon_sym_TRUE = 14,
  anon_sym_FALSE = 15,
  anon_sym_BSLASH = 16,
  aux_sym_tla_escape_char_token1 = 17,
  sym_tla_nat_number = 18,
  sym_tla_real_number = 19,
  anon_sym_BSLASHb = 20,
  anon_sym_BSLASHB = 21,
  aux_sym_tla_binary_number_token1 = 22,
  anon_sym_BSLASHo = 23,
  anon_sym_BSLASHO = 24,
  aux_sym_tla_octal_number_token1 = 25,
  anon_sym_BSLASHh = 26,
  anon_sym_BSLASHH = 27,
  aux_sym_tla_hex_number_token1 = 28,
  sym_tla_all_map_to = 29,
  sym_tla_langle_bracket = 30,
  sym_tla_rangle_bracket = 31,
  sym_tla_map_to = 32,
  sym_tla_variable = 33,
  sym_tla_conjunction = 34,
  sym_tla_state_conjunction_variable_value_list = 35,
  sym_tla_conjunction_variable_value = 36,
  sym_tla_variable_value = 37,
  sym_tla_record = 38,
  sym_tla_record_item_comma_list = 39,
  sym_tla_record_field = 40,
  sym_tla_value_expr = 41,
  sym_tla_function = 42,
  sym_tla_function_comma_list = 43,
  sym_tla_mapping = 44,
  sym_tla_finite_set = 45,
  sym_tla_finite_set_item_comma_list = 46,
  sym_tla_sequence = 47,
  sym_tla_sequence_item_comma_list = 48,
  sym_tla_string = 49,
  sym_tla_boolean = 50,
  sym_tla_escape_char = 51,
  sym_tla_number = 52,
  sym_tla_binary_number = 53,
  sym_tla_octal_number = 54,
  sym_tla_hex_number = 55,
  aux_sym_tla_state_conjunction_variable_value_list_repeat1 = 56,
  aux_sym_tla_record_item_comma_list_repeat1 = 57,
  aux_sym_tla_function_comma_list_repeat1 = 58,
  aux_sym_tla_finite_set_item_comma_list_repeat1 = 59,
  aux_sym_tla_string_repeat1 = 60,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_EQ] = "=",
  [sym_tla_action_name] = "tla_action_name",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_COMMA] = ",",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_AT_AT] = "@@",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_tla_string_token1] = "tla_string_token1",
  [anon_sym_DQUOTE2] = "\"",
  [anon_sym_TRUE] = "TRUE",
  [anon_sym_FALSE] = "FALSE",
  [anon_sym_BSLASH] = "\\",
  [aux_sym_tla_escape_char_token1] = "tla_escape_char_token1",
  [sym_tla_nat_number] = "tla_nat_number",
  [sym_tla_real_number] = "tla_real_number",
  [anon_sym_BSLASHb] = "format",
  [anon_sym_BSLASHB] = "format",
  [aux_sym_tla_binary_number_token1] = "value",
  [anon_sym_BSLASHo] = "format",
  [anon_sym_BSLASHO] = "format",
  [aux_sym_tla_octal_number_token1] = "value",
  [anon_sym_BSLASHh] = "format",
  [anon_sym_BSLASHH] = "format",
  [aux_sym_tla_hex_number_token1] = "value",
  [sym_tla_all_map_to] = "tla_all_map_to",
  [sym_tla_langle_bracket] = "tla_langle_bracket",
  [sym_tla_rangle_bracket] = "tla_rangle_bracket",
  [sym_tla_map_to] = "tla_map_to",
  [sym_tla_variable] = "tla_variable",
  [sym_tla_conjunction] = "tla_conjunction",
  [sym_tla_state_conjunction_variable_value_list] = "tla_state_conjunction_variable_value_list",
  [sym_tla_conjunction_variable_value] = "tla_conjunction_variable_value",
  [sym_tla_variable_value] = "tla_variable_value",
  [sym_tla_record] = "tla_record",
  [sym_tla_record_item_comma_list] = "tla_record_item_comma_list",
  [sym_tla_record_field] = "tla_record_field",
  [sym_tla_value_expr] = "tla_value_expr",
  [sym_tla_function] = "tla_function",
  [sym_tla_function_comma_list] = "tla_function_comma_list",
  [sym_tla_mapping] = "tla_mapping",
  [sym_tla_finite_set] = "tla_finite_set",
  [sym_tla_finite_set_item_comma_list] = "tla_finite_set_item_comma_list",
  [sym_tla_sequence] = "tla_sequence",
  [sym_tla_sequence_item_comma_list] = "tla_sequence_item_comma_list",
  [sym_tla_string] = "tla_string",
  [sym_tla_boolean] = "tla_boolean",
  [sym_tla_escape_char] = "tla_escape_char",
  [sym_tla_number] = "tla_number",
  [sym_tla_binary_number] = "tla_binary_number",
  [sym_tla_octal_number] = "tla_octal_number",
  [sym_tla_hex_number] = "tla_hex_number",
  [aux_sym_tla_state_conjunction_variable_value_list_repeat1] = "tla_state_conjunction_variable_value_list_repeat1",
  [aux_sym_tla_record_item_comma_list_repeat1] = "tla_record_item_comma_list_repeat1",
  [aux_sym_tla_function_comma_list_repeat1] = "tla_function_comma_list_repeat1",
  [aux_sym_tla_finite_set_item_comma_list_repeat1] = "tla_finite_set_item_comma_list_repeat1",
  [aux_sym_tla_string_repeat1] = "tla_string_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_EQ] = anon_sym_EQ,
  [sym_tla_action_name] = sym_tla_action_name,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_AT_AT] = anon_sym_AT_AT,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_tla_string_token1] = aux_sym_tla_string_token1,
  [anon_sym_DQUOTE2] = anon_sym_DQUOTE,
  [anon_sym_TRUE] = anon_sym_TRUE,
  [anon_sym_FALSE] = anon_sym_FALSE,
  [anon_sym_BSLASH] = anon_sym_BSLASH,
  [aux_sym_tla_escape_char_token1] = aux_sym_tla_escape_char_token1,
  [sym_tla_nat_number] = sym_tla_nat_number,
  [sym_tla_real_number] = sym_tla_real_number,
  [anon_sym_BSLASHb] = anon_sym_BSLASHb,
  [anon_sym_BSLASHB] = anon_sym_BSLASHb,
  [aux_sym_tla_binary_number_token1] = aux_sym_tla_binary_number_token1,
  [anon_sym_BSLASHo] = anon_sym_BSLASHb,
  [anon_sym_BSLASHO] = anon_sym_BSLASHb,
  [aux_sym_tla_octal_number_token1] = aux_sym_tla_binary_number_token1,
  [anon_sym_BSLASHh] = anon_sym_BSLASHb,
  [anon_sym_BSLASHH] = anon_sym_BSLASHb,
  [aux_sym_tla_hex_number_token1] = aux_sym_tla_binary_number_token1,
  [sym_tla_all_map_to] = sym_tla_all_map_to,
  [sym_tla_langle_bracket] = sym_tla_langle_bracket,
  [sym_tla_rangle_bracket] = sym_tla_rangle_bracket,
  [sym_tla_map_to] = sym_tla_map_to,
  [sym_tla_variable] = sym_tla_variable,
  [sym_tla_conjunction] = sym_tla_conjunction,
  [sym_tla_state_conjunction_variable_value_list] = sym_tla_state_conjunction_variable_value_list,
  [sym_tla_conjunction_variable_value] = sym_tla_conjunction_variable_value,
  [sym_tla_variable_value] = sym_tla_variable_value,
  [sym_tla_record] = sym_tla_record,
  [sym_tla_record_item_comma_list] = sym_tla_record_item_comma_list,
  [sym_tla_record_field] = sym_tla_record_field,
  [sym_tla_value_expr] = sym_tla_value_expr,
  [sym_tla_function] = sym_tla_function,
  [sym_tla_function_comma_list] = sym_tla_function_comma_list,
  [sym_tla_mapping] = sym_tla_mapping,
  [sym_tla_finite_set] = sym_tla_finite_set,
  [sym_tla_finite_set_item_comma_list] = sym_tla_finite_set_item_comma_list,
  [sym_tla_sequence] = sym_tla_sequence,
  [sym_tla_sequence_item_comma_list] = sym_tla_sequence_item_comma_list,
  [sym_tla_string] = sym_tla_string,
  [sym_tla_boolean] = sym_tla_boolean,
  [sym_tla_escape_char] = sym_tla_escape_char,
  [sym_tla_number] = sym_tla_number,
  [sym_tla_binary_number] = sym_tla_binary_number,
  [sym_tla_octal_number] = sym_tla_octal_number,
  [sym_tla_hex_number] = sym_tla_hex_number,
  [aux_sym_tla_state_conjunction_variable_value_list_repeat1] = aux_sym_tla_state_conjunction_variable_value_list_repeat1,
  [aux_sym_tla_record_item_comma_list_repeat1] = aux_sym_tla_record_item_comma_list_repeat1,
  [aux_sym_tla_function_comma_list_repeat1] = aux_sym_tla_function_comma_list_repeat1,
  [aux_sym_tla_finite_set_item_comma_list_repeat1] = aux_sym_tla_finite_set_item_comma_list_repeat1,
  [aux_sym_tla_string_repeat1] = aux_sym_tla_string_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [sym_tla_action_name] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_tla_string_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DQUOTE2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TRUE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_FALSE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BSLASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_tla_escape_char_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_tla_nat_number] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_real_number] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_BSLASHb] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_BSLASHB] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_tla_binary_number_token1] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_BSLASHo] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_BSLASHO] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_tla_octal_number_token1] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_BSLASHh] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_BSLASHH] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_tla_hex_number_token1] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_all_map_to] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_langle_bracket] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_rangle_bracket] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_map_to] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_variable] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_conjunction] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_state_conjunction_variable_value_list] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_conjunction_variable_value] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_variable_value] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_record] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_record_item_comma_list] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_record_field] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_value_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_function] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_function_comma_list] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_mapping] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_finite_set] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_finite_set_item_comma_list] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_sequence] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_sequence_item_comma_list] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_string] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_boolean] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_escape_char] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_number] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_binary_number] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_octal_number] = {
    .visible = true,
    .named = true,
  },
  [sym_tla_hex_number] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_tla_state_conjunction_variable_value_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_tla_record_item_comma_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_tla_function_comma_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_tla_finite_set_item_comma_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_tla_string_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_conjunction = 1,
  field_tla_binary_number = 2,
  field_tla_boolean = 3,
  field_tla_finite_set = 4,
  field_tla_finite_set_item_comma_list = 5,
  field_tla_function = 6,
  field_tla_function_comma_list = 7,
  field_tla_hex_number = 8,
  field_tla_map_to = 9,
  field_tla_nat_number = 10,
  field_tla_number = 11,
  field_tla_octal_number = 12,
  field_tla_real_number = 13,
  field_tla_record = 14,
  field_tla_record_field_left = 15,
  field_tla_record_field_right = 16,
  field_tla_record_item_comma_list = 17,
  field_tla_record_variable_map_to_to = 18,
  field_tla_sequence = 19,
  field_tla_sequence_item_comma_list = 20,
  field_tla_string = 21,
  field_tla_variable = 22,
  field_tla_variable_value = 23,
  field_tla_variable_value_left = 24,
  field_tla_variable_value_right = 25,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_conjunction] = "conjunction",
  [field_tla_binary_number] = "tla_binary_number",
  [field_tla_boolean] = "tla_boolean",
  [field_tla_finite_set] = "tla_finite_set",
  [field_tla_finite_set_item_comma_list] = "tla_finite_set_item_comma_list",
  [field_tla_function] = "tla_function",
  [field_tla_function_comma_list] = "tla_function_comma_list",
  [field_tla_hex_number] = "tla_hex_number",
  [field_tla_map_to] = "tla_map_to",
  [field_tla_nat_number] = "tla_nat_number",
  [field_tla_number] = "tla_number",
  [field_tla_octal_number] = "tla_octal_number",
  [field_tla_real_number] = "tla_real_number",
  [field_tla_record] = "tla_record",
  [field_tla_record_field_left] = "tla_record_field_left",
  [field_tla_record_field_right] = "tla_record_field_right",
  [field_tla_record_item_comma_list] = "tla_record_item_comma_list",
  [field_tla_record_variable_map_to_to] = "tla_record_variable_map_to_to",
  [field_tla_sequence] = "tla_sequence",
  [field_tla_sequence_item_comma_list] = "tla_sequence_item_comma_list",
  [field_tla_string] = "tla_string",
  [field_tla_variable] = "tla_variable",
  [field_tla_variable_value] = "tla_variable_value",
  [field_tla_variable_value_left] = "tla_variable_value_left",
  [field_tla_variable_value_right] = "tla_variable_value_right",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 1},
  [3] = {.index = 3, .length = 1},
  [4] = {.index = 4, .length = 1},
  [5] = {.index = 5, .length = 1},
  [6] = {.index = 6, .length = 2},
  [7] = {.index = 8, .length = 1},
  [8] = {.index = 9, .length = 1},
  [9] = {.index = 10, .length = 1},
  [10] = {.index = 11, .length = 1},
  [11] = {.index = 12, .length = 1},
  [12] = {.index = 13, .length = 1},
  [13] = {.index = 14, .length = 1},
  [14] = {.index = 15, .length = 1},
  [15] = {.index = 16, .length = 1},
  [16] = {.index = 17, .length = 1},
  [17] = {.index = 18, .length = 1},
  [18] = {.index = 19, .length = 1},
  [19] = {.index = 20, .length = 1},
  [20] = {.index = 21, .length = 3},
  [21] = {.index = 24, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_conjunction, 0},
    {field_tla_variable_value, 1},
  [2] =
    {field_tla_nat_number, 0},
  [3] =
    {field_tla_real_number, 0},
  [4] =
    {field_tla_variable, 0},
  [5] =
    {field_tla_record, 0},
  [6] =
    {field_tla_variable_value_left, 0},
    {field_tla_variable_value_right, 2},
  [8] =
    {field_tla_function, 0},
  [9] =
    {field_tla_finite_set, 0},
  [10] =
    {field_tla_sequence, 0},
  [11] =
    {field_tla_string, 0},
  [12] =
    {field_tla_boolean, 0},
  [13] =
    {field_tla_number, 0},
  [14] =
    {field_tla_binary_number, 0},
  [15] =
    {field_tla_octal_number, 0},
  [16] =
    {field_tla_hex_number, 0},
  [17] =
    {field_tla_record_item_comma_list, 1},
  [18] =
    {field_tla_function_comma_list, 1},
  [19] =
    {field_tla_finite_set_item_comma_list, 1},
  [20] =
    {field_tla_sequence_item_comma_list, 1},
  [21] =
    {field_tla_record_field_left, 0},
    {field_tla_record_field_right, 2},
    {field_tla_record_variable_map_to_to, 1},
  [24] =
    {field_tla_map_to, 1},
    {field_tla_record_field_left, 0},
    {field_tla_record_field_right, 2},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  switch (state) {
    case 0:
      if (eof) ADVANCE(20);
      if (lookahead == '"') ADVANCE(41);
      if (lookahead == '(') ADVANCE(34);
      if (lookahead == ')') ADVANCE(35);
      if (lookahead == ',') ADVANCE(33);
      if (lookahead == '.') ADVANCE(15);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(5);
      if (lookahead == '<') ADVANCE(4);
      if (lookahead == '=') ADVANCE(21);
      if (lookahead == '>') ADVANCE(6);
      if (lookahead == '@') ADVANCE(8);
      if (lookahead == 'F') ADVANCE(63);
      if (lookahead == 'T') ADVANCE(68);
      if (lookahead == '[') ADVANCE(31);
      if (lookahead == '\\') ADVANCE(46);
      if (lookahead == ']') ADVANCE(32);
      if (lookahead == '_') ADVANCE(12);
      if (lookahead == '{') ADVANCE(37);
      if (lookahead == '|') ADVANCE(3);
      if (lookahead == '}') ADVANCE(38);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(19)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(48);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(72);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(41);
      if (lookahead == '\\') ADVANCE(46);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(40);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(39);
      if (lookahead == '(') ADVANCE(34);
      if (lookahead == ')') ADVANCE(35);
      if (lookahead == '.') ADVANCE(15);
      if (lookahead == '<') ADVANCE(4);
      if (lookahead == '>') ADVANCE(6);
      if (lookahead == 'F') ADVANCE(64);
      if (lookahead == 'T') ADVANCE(69);
      if (lookahead == '[') ADVANCE(31);
      if (lookahead == '\\') ADVANCE(9);
      if (lookahead == '_') ADVANCE(16);
      if (lookahead == '{') ADVANCE(37);
      if (lookahead == '}') ADVANCE(38);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(48);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(7);
      END_STATE();
    case 4:
      if (lookahead == '<') ADVANCE(60);
      END_STATE();
    case 5:
      if (lookahead == '>') ADVANCE(62);
      END_STATE();
    case 6:
      if (lookahead == '>') ADVANCE(61);
      END_STATE();
    case 7:
      if (lookahead == '>') ADVANCE(59);
      END_STATE();
    case 8:
      if (lookahead == '@') ADVANCE(36);
      END_STATE();
    case 9:
      if (lookahead == 'B') ADVANCE(51);
      if (lookahead == 'H') ADVANCE(57);
      if (lookahead == 'O') ADVANCE(54);
      if (lookahead == 'b') ADVANCE(50);
      if (lookahead == 'h') ADVANCE(56);
      if (lookahead == 'o') ADVANCE(53);
      END_STATE();
    case 10:
      if (lookahead == '\\') ADVANCE(74);
      END_STATE();
    case 11:
      if (lookahead == ']') ADVANCE(32);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(11)
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(16);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 12:
      if (lookahead == '_') ADVANCE(27);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(29);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 13:
      if (lookahead == '0' ||
          lookahead == '1') ADVANCE(52);
      END_STATE();
    case 14:
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(55);
      END_STATE();
    case 15:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(49);
      END_STATE();
    case 16:
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(16);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 17:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(58);
      END_STATE();
    case 18:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(47);
      END_STATE();
    case 19:
      if (eof) ADVANCE(20);
      if (lookahead == '"') ADVANCE(39);
      if (lookahead == '(') ADVANCE(34);
      if (lookahead == ')') ADVANCE(35);
      if (lookahead == ',') ADVANCE(33);
      if (lookahead == '.') ADVANCE(15);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(5);
      if (lookahead == '<') ADVANCE(4);
      if (lookahead == '=') ADVANCE(21);
      if (lookahead == '>') ADVANCE(6);
      if (lookahead == '@') ADVANCE(8);
      if (lookahead == 'F') ADVANCE(63);
      if (lookahead == 'T') ADVANCE(68);
      if (lookahead == '[') ADVANCE(31);
      if (lookahead == ']') ADVANCE(32);
      if (lookahead == '_') ADVANCE(12);
      if (lookahead == '{') ADVANCE(37);
      if (lookahead == '|') ADVANCE(3);
      if (lookahead == '}') ADVANCE(38);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(19)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(48);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(72);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_tla_action_name);
      if (lookahead == 'E') ADVANCE(42);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_tla_action_name);
      if (lookahead == 'E') ADVANCE(44);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_tla_action_name);
      if (lookahead == 'L') ADVANCE(25);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_tla_action_name);
      if (lookahead == 'S') ADVANCE(23);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_tla_action_name);
      if (lookahead == 'U') ADVANCE(22);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_tla_action_name);
      if (lookahead == '_') ADVANCE(27);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(29);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_tla_action_name);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_tla_action_name);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(29);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_tla_action_name);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_AT_AT);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(aux_sym_tla_string_token1);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_DQUOTE2);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_TRUE);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_TRUE);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_FALSE);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_FALSE);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_BSLASH);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(aux_sym_tla_escape_char_token1);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_tla_nat_number);
      if (lookahead == '.') ADVANCE(15);
      if (lookahead == '_') ADVANCE(16);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(48);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_tla_real_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(49);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_BSLASHb);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_BSLASHB);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(aux_sym_tla_binary_number_token1);
      if (lookahead == '0' ||
          lookahead == '1') ADVANCE(52);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(anon_sym_BSLASHo);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_BSLASHO);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(aux_sym_tla_octal_number_token1);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(55);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_BSLASHh);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(anon_sym_BSLASHH);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(aux_sym_tla_hex_number_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(58);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_tla_all_map_to);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_tla_langle_bracket);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(sym_tla_rangle_bracket);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(sym_tla_map_to);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(sym_tla_variable);
      if (lookahead == 'A') ADVANCE(24);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(sym_tla_variable);
      if (lookahead == 'A') ADVANCE(67);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(sym_tla_variable);
      if (lookahead == 'E') ADVANCE(43);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(sym_tla_variable);
      if (lookahead == 'E') ADVANCE(45);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_tla_variable);
      if (lookahead == 'L') ADVANCE(70);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_tla_variable);
      if (lookahead == 'R') ADVANCE(26);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_tla_variable);
      if (lookahead == 'R') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_tla_variable);
      if (lookahead == 'S') ADVANCE(66);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_tla_variable);
      if (lookahead == 'U') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_tla_variable);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_tla_variable);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_tla_conjunction);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 2},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 2},
  [8] = {.lex_state = 2},
  [9] = {.lex_state = 2},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 1},
  [38] = {.lex_state = 1},
  [39] = {.lex_state = 1},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 11},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 0},
  [52] = {.lex_state = 0},
  [53] = {.lex_state = 1},
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 0},
  [56] = {.lex_state = 11},
  [57] = {.lex_state = 0},
  [58] = {.lex_state = 11},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 0},
  [63] = {.lex_state = 0},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 18},
  [68] = {.lex_state = 0},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 17},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 13},
  [75] = {.lex_state = 14},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [sym_tla_action_name] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_AT_AT] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE2] = ACTIONS(1),
    [anon_sym_TRUE] = ACTIONS(1),
    [anon_sym_FALSE] = ACTIONS(1),
    [anon_sym_BSLASH] = ACTIONS(1),
    [sym_tla_nat_number] = ACTIONS(1),
    [sym_tla_real_number] = ACTIONS(1),
    [sym_tla_all_map_to] = ACTIONS(1),
    [sym_tla_langle_bracket] = ACTIONS(1),
    [sym_tla_rangle_bracket] = ACTIONS(1),
    [sym_tla_map_to] = ACTIONS(1),
    [sym_tla_variable] = ACTIONS(1),
    [sym_tla_conjunction] = ACTIONS(1),
  },
  [1] = {
    [sym_tla_state_conjunction_variable_value_list] = STATE(66),
    [sym_tla_conjunction_variable_value] = STATE(40),
    [aux_sym_tla_state_conjunction_variable_value_list_repeat1] = STATE(40),
    [sym_tla_conjunction] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 26,
    ACTIONS(5), 1,
      anon_sym_LBRACK,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      anon_sym_RPAREN,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      sym_tla_nat_number,
    ACTIONS(19), 1,
      sym_tla_real_number,
    ACTIONS(27), 1,
      sym_tla_langle_bracket,
    ACTIONS(29), 1,
      sym_tla_variable,
    STATE(11), 1,
      sym_tla_boolean,
    STATE(21), 1,
      sym_tla_record,
    STATE(23), 1,
      sym_tla_function,
    STATE(24), 1,
      sym_tla_finite_set,
    STATE(25), 1,
      sym_tla_sequence,
    STATE(26), 1,
      sym_tla_string,
    STATE(28), 1,
      sym_tla_number,
    STATE(29), 1,
      sym_tla_binary_number,
    STATE(30), 1,
      sym_tla_octal_number,
    STATE(31), 1,
      sym_tla_hex_number,
    STATE(49), 1,
      sym_tla_mapping,
    STATE(65), 1,
      sym_tla_value_expr,
    STATE(70), 1,
      sym_tla_function_comma_list,
    ACTIONS(15), 2,
      anon_sym_TRUE,
      anon_sym_FALSE,
    ACTIONS(21), 2,
      anon_sym_BSLASHb,
      anon_sym_BSLASHB,
    ACTIONS(23), 2,
      anon_sym_BSLASHo,
      anon_sym_BSLASHO,
    ACTIONS(25), 2,
      anon_sym_BSLASHh,
      anon_sym_BSLASHH,
  [83] = 25,
    ACTIONS(5), 1,
      anon_sym_LBRACK,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      sym_tla_nat_number,
    ACTIONS(19), 1,
      sym_tla_real_number,
    ACTIONS(27), 1,
      sym_tla_langle_bracket,
    ACTIONS(29), 1,
      sym_tla_variable,
    ACTIONS(31), 1,
      anon_sym_RBRACE,
    STATE(11), 1,
      sym_tla_boolean,
    STATE(21), 1,
      sym_tla_record,
    STATE(23), 1,
      sym_tla_function,
    STATE(24), 1,
      sym_tla_finite_set,
    STATE(25), 1,
      sym_tla_sequence,
    STATE(26), 1,
      sym_tla_string,
    STATE(28), 1,
      sym_tla_number,
    STATE(29), 1,
      sym_tla_binary_number,
    STATE(30), 1,
      sym_tla_octal_number,
    STATE(31), 1,
      sym_tla_hex_number,
    STATE(48), 1,
      sym_tla_value_expr,
    STATE(72), 1,
      sym_tla_finite_set_item_comma_list,
    ACTIONS(15), 2,
      anon_sym_TRUE,
      anon_sym_FALSE,
    ACTIONS(21), 2,
      anon_sym_BSLASHb,
      anon_sym_BSLASHB,
    ACTIONS(23), 2,
      anon_sym_BSLASHo,
      anon_sym_BSLASHO,
    ACTIONS(25), 2,
      anon_sym_BSLASHh,
      anon_sym_BSLASHH,
  [163] = 25,
    ACTIONS(5), 1,
      anon_sym_LBRACK,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      sym_tla_nat_number,
    ACTIONS(19), 1,
      sym_tla_real_number,
    ACTIONS(27), 1,
      sym_tla_langle_bracket,
    ACTIONS(29), 1,
      sym_tla_variable,
    ACTIONS(33), 1,
      sym_tla_rangle_bracket,
    STATE(11), 1,
      sym_tla_boolean,
    STATE(21), 1,
      sym_tla_record,
    STATE(23), 1,
      sym_tla_function,
    STATE(24), 1,
      sym_tla_finite_set,
    STATE(25), 1,
      sym_tla_sequence,
    STATE(26), 1,
      sym_tla_string,
    STATE(28), 1,
      sym_tla_number,
    STATE(29), 1,
      sym_tla_binary_number,
    STATE(30), 1,
      sym_tla_octal_number,
    STATE(31), 1,
      sym_tla_hex_number,
    STATE(50), 1,
      sym_tla_value_expr,
    STATE(73), 1,
      sym_tla_sequence_item_comma_list,
    ACTIONS(15), 2,
      anon_sym_TRUE,
      anon_sym_FALSE,
    ACTIONS(21), 2,
      anon_sym_BSLASHb,
      anon_sym_BSLASHB,
    ACTIONS(23), 2,
      anon_sym_BSLASHo,
      anon_sym_BSLASHO,
    ACTIONS(25), 2,
      anon_sym_BSLASHh,
      anon_sym_BSLASHH,
  [243] = 24,
    ACTIONS(5), 1,
      anon_sym_LBRACK,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      sym_tla_nat_number,
    ACTIONS(19), 1,
      sym_tla_real_number,
    ACTIONS(27), 1,
      sym_tla_langle_bracket,
    ACTIONS(29), 1,
      sym_tla_variable,
    STATE(11), 1,
      sym_tla_boolean,
    STATE(21), 1,
      sym_tla_record,
    STATE(23), 1,
      sym_tla_function,
    STATE(24), 1,
      sym_tla_finite_set,
    STATE(25), 1,
      sym_tla_sequence,
    STATE(26), 1,
      sym_tla_string,
    STATE(28), 1,
      sym_tla_number,
    STATE(29), 1,
      sym_tla_binary_number,
    STATE(30), 1,
      sym_tla_octal_number,
    STATE(31), 1,
      sym_tla_hex_number,
    STATE(63), 1,
      sym_tla_mapping,
    STATE(65), 1,
      sym_tla_value_expr,
    ACTIONS(15), 2,
      anon_sym_TRUE,
      anon_sym_FALSE,
    ACTIONS(21), 2,
      anon_sym_BSLASHb,
      anon_sym_BSLASHB,
    ACTIONS(23), 2,
      anon_sym_BSLASHo,
      anon_sym_BSLASHO,
    ACTIONS(25), 2,
      anon_sym_BSLASHh,
      anon_sym_BSLASHH,
  [320] = 23,
    ACTIONS(5), 1,
      anon_sym_LBRACK,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      sym_tla_nat_number,
    ACTIONS(19), 1,
      sym_tla_real_number,
    ACTIONS(27), 1,
      sym_tla_langle_bracket,
    ACTIONS(29), 1,
      sym_tla_variable,
    STATE(11), 1,
      sym_tla_boolean,
    STATE(21), 1,
      sym_tla_record,
    STATE(23), 1,
      sym_tla_function,
    STATE(24), 1,
      sym_tla_finite_set,
    STATE(25), 1,
      sym_tla_sequence,
    STATE(26), 1,
      sym_tla_string,
    STATE(28), 1,
      sym_tla_number,
    STATE(29), 1,
      sym_tla_binary_number,
    STATE(30), 1,
      sym_tla_octal_number,
    STATE(31), 1,
      sym_tla_hex_number,
    STATE(46), 1,
      sym_tla_value_expr,
    ACTIONS(15), 2,
      anon_sym_TRUE,
      anon_sym_FALSE,
    ACTIONS(21), 2,
      anon_sym_BSLASHb,
      anon_sym_BSLASHB,
    ACTIONS(23), 2,
      anon_sym_BSLASHo,
      anon_sym_BSLASHO,
    ACTIONS(25), 2,
      anon_sym_BSLASHh,
      anon_sym_BSLASHH,
  [394] = 23,
    ACTIONS(5), 1,
      anon_sym_LBRACK,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      sym_tla_nat_number,
    ACTIONS(19), 1,
      sym_tla_real_number,
    ACTIONS(27), 1,
      sym_tla_langle_bracket,
    ACTIONS(29), 1,
      sym_tla_variable,
    STATE(11), 1,
      sym_tla_boolean,
    STATE(21), 1,
      sym_tla_record,
    STATE(23), 1,
      sym_tla_function,
    STATE(24), 1,
      sym_tla_finite_set,
    STATE(25), 1,
      sym_tla_sequence,
    STATE(26), 1,
      sym_tla_string,
    STATE(28), 1,
      sym_tla_number,
    STATE(29), 1,
      sym_tla_binary_number,
    STATE(30), 1,
      sym_tla_octal_number,
    STATE(31), 1,
      sym_tla_hex_number,
    STATE(62), 1,
      sym_tla_value_expr,
    ACTIONS(15), 2,
      anon_sym_TRUE,
      anon_sym_FALSE,
    ACTIONS(21), 2,
      anon_sym_BSLASHb,
      anon_sym_BSLASHB,
    ACTIONS(23), 2,
      anon_sym_BSLASHo,
      anon_sym_BSLASHO,
    ACTIONS(25), 2,
      anon_sym_BSLASHh,
      anon_sym_BSLASHH,
  [468] = 23,
    ACTIONS(5), 1,
      anon_sym_LBRACK,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      sym_tla_nat_number,
    ACTIONS(19), 1,
      sym_tla_real_number,
    ACTIONS(27), 1,
      sym_tla_langle_bracket,
    ACTIONS(29), 1,
      sym_tla_variable,
    STATE(11), 1,
      sym_tla_boolean,
    STATE(21), 1,
      sym_tla_record,
    STATE(23), 1,
      sym_tla_function,
    STATE(24), 1,
      sym_tla_finite_set,
    STATE(25), 1,
      sym_tla_sequence,
    STATE(26), 1,
      sym_tla_string,
    STATE(28), 1,
      sym_tla_number,
    STATE(29), 1,
      sym_tla_binary_number,
    STATE(30), 1,
      sym_tla_octal_number,
    STATE(31), 1,
      sym_tla_hex_number,
    STATE(60), 1,
      sym_tla_value_expr,
    ACTIONS(15), 2,
      anon_sym_TRUE,
      anon_sym_FALSE,
    ACTIONS(21), 2,
      anon_sym_BSLASHb,
      anon_sym_BSLASHB,
    ACTIONS(23), 2,
      anon_sym_BSLASHo,
      anon_sym_BSLASHO,
    ACTIONS(25), 2,
      anon_sym_BSLASHh,
      anon_sym_BSLASHH,
  [542] = 23,
    ACTIONS(5), 1,
      anon_sym_LBRACK,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      sym_tla_nat_number,
    ACTIONS(19), 1,
      sym_tla_real_number,
    ACTIONS(27), 1,
      sym_tla_langle_bracket,
    ACTIONS(29), 1,
      sym_tla_variable,
    STATE(11), 1,
      sym_tla_boolean,
    STATE(21), 1,
      sym_tla_record,
    STATE(23), 1,
      sym_tla_function,
    STATE(24), 1,
      sym_tla_finite_set,
    STATE(25), 1,
      sym_tla_sequence,
    STATE(26), 1,
      sym_tla_string,
    STATE(28), 1,
      sym_tla_number,
    STATE(29), 1,
      sym_tla_binary_number,
    STATE(30), 1,
      sym_tla_octal_number,
    STATE(31), 1,
      sym_tla_hex_number,
    STATE(59), 1,
      sym_tla_value_expr,
    ACTIONS(15), 2,
      anon_sym_TRUE,
      anon_sym_FALSE,
    ACTIONS(21), 2,
      anon_sym_BSLASHb,
      anon_sym_BSLASHB,
    ACTIONS(23), 2,
      anon_sym_BSLASHo,
      anon_sym_BSLASHO,
    ACTIONS(25), 2,
      anon_sym_BSLASHh,
      anon_sym_BSLASHH,
  [616] = 1,
    ACTIONS(35), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [628] = 1,
    ACTIONS(37), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [640] = 1,
    ACTIONS(39), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [652] = 1,
    ACTIONS(41), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [664] = 1,
    ACTIONS(43), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [676] = 1,
    ACTIONS(45), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [688] = 1,
    ACTIONS(47), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [700] = 1,
    ACTIONS(49), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [712] = 1,
    ACTIONS(51), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [724] = 1,
    ACTIONS(53), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [736] = 1,
    ACTIONS(55), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [748] = 1,
    ACTIONS(57), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [760] = 1,
    ACTIONS(59), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [772] = 1,
    ACTIONS(61), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [784] = 1,
    ACTIONS(63), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [796] = 1,
    ACTIONS(65), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [808] = 1,
    ACTIONS(67), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [820] = 1,
    ACTIONS(69), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [832] = 1,
    ACTIONS(71), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [844] = 1,
    ACTIONS(73), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [856] = 1,
    ACTIONS(75), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [868] = 1,
    ACTIONS(77), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [880] = 1,
    ACTIONS(79), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [892] = 1,
    ACTIONS(81), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [904] = 1,
    ACTIONS(83), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [916] = 1,
    ACTIONS(85), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [928] = 1,
    ACTIONS(87), 9,
      ts_builtin_sym_end,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
      sym_tla_map_to,
      sym_tla_conjunction,
  [940] = 4,
    ACTIONS(89), 1,
      aux_sym_tla_string_token1,
    ACTIONS(92), 1,
      anon_sym_DQUOTE2,
    ACTIONS(94), 1,
      anon_sym_BSLASH,
    STATE(37), 2,
      sym_tla_escape_char,
      aux_sym_tla_string_repeat1,
  [954] = 4,
    ACTIONS(97), 1,
      aux_sym_tla_string_token1,
    ACTIONS(99), 1,
      anon_sym_DQUOTE2,
    ACTIONS(101), 1,
      anon_sym_BSLASH,
    STATE(39), 2,
      sym_tla_escape_char,
      aux_sym_tla_string_repeat1,
  [968] = 4,
    ACTIONS(101), 1,
      anon_sym_BSLASH,
    ACTIONS(103), 1,
      aux_sym_tla_string_token1,
    ACTIONS(105), 1,
      anon_sym_DQUOTE2,
    STATE(37), 2,
      sym_tla_escape_char,
      aux_sym_tla_string_repeat1,
  [982] = 3,
    ACTIONS(3), 1,
      sym_tla_conjunction,
    ACTIONS(107), 1,
      ts_builtin_sym_end,
    STATE(43), 2,
      sym_tla_conjunction_variable_value,
      aux_sym_tla_state_conjunction_variable_value_list_repeat1,
  [993] = 3,
    ACTIONS(109), 1,
      anon_sym_COMMA,
    STATE(41), 1,
      aux_sym_tla_finite_set_item_comma_list_repeat1,
    ACTIONS(112), 2,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
  [1004] = 4,
    ACTIONS(114), 1,
      anon_sym_RBRACK,
    ACTIONS(116), 1,
      sym_tla_variable,
    STATE(45), 1,
      sym_tla_record_field,
    STATE(68), 1,
      sym_tla_record_item_comma_list,
  [1017] = 3,
    ACTIONS(118), 1,
      ts_builtin_sym_end,
    ACTIONS(120), 1,
      sym_tla_conjunction,
    STATE(43), 2,
      sym_tla_conjunction_variable_value,
      aux_sym_tla_state_conjunction_variable_value_list_repeat1,
  [1028] = 3,
    ACTIONS(123), 1,
      anon_sym_RPAREN,
    ACTIONS(125), 1,
      anon_sym_AT_AT,
    STATE(47), 1,
      aux_sym_tla_function_comma_list_repeat1,
  [1038] = 3,
    ACTIONS(127), 1,
      anon_sym_RBRACK,
    ACTIONS(129), 1,
      anon_sym_COMMA,
    STATE(55), 1,
      aux_sym_tla_record_item_comma_list_repeat1,
  [1048] = 1,
    ACTIONS(112), 3,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      sym_tla_rangle_bracket,
  [1054] = 3,
    ACTIONS(131), 1,
      anon_sym_RPAREN,
    ACTIONS(133), 1,
      anon_sym_AT_AT,
    STATE(47), 1,
      aux_sym_tla_function_comma_list_repeat1,
  [1064] = 3,
    ACTIONS(136), 1,
      anon_sym_COMMA,
    ACTIONS(138), 1,
      anon_sym_RBRACE,
    STATE(54), 1,
      aux_sym_tla_finite_set_item_comma_list_repeat1,
  [1074] = 3,
    ACTIONS(125), 1,
      anon_sym_AT_AT,
    ACTIONS(140), 1,
      anon_sym_RPAREN,
    STATE(44), 1,
      aux_sym_tla_function_comma_list_repeat1,
  [1084] = 3,
    ACTIONS(136), 1,
      anon_sym_COMMA,
    ACTIONS(142), 1,
      sym_tla_rangle_bracket,
    STATE(52), 1,
      aux_sym_tla_finite_set_item_comma_list_repeat1,
  [1094] = 3,
    ACTIONS(144), 1,
      anon_sym_RBRACK,
    ACTIONS(146), 1,
      anon_sym_COMMA,
    STATE(51), 1,
      aux_sym_tla_record_item_comma_list_repeat1,
  [1104] = 3,
    ACTIONS(136), 1,
      anon_sym_COMMA,
    ACTIONS(149), 1,
      sym_tla_rangle_bracket,
    STATE(41), 1,
      aux_sym_tla_finite_set_item_comma_list_repeat1,
  [1114] = 2,
    ACTIONS(151), 1,
      aux_sym_tla_string_token1,
    ACTIONS(153), 2,
      anon_sym_DQUOTE2,
      anon_sym_BSLASH,
  [1122] = 3,
    ACTIONS(136), 1,
      anon_sym_COMMA,
    ACTIONS(155), 1,
      anon_sym_RBRACE,
    STATE(41), 1,
      aux_sym_tla_finite_set_item_comma_list_repeat1,
  [1132] = 3,
    ACTIONS(129), 1,
      anon_sym_COMMA,
    ACTIONS(157), 1,
      anon_sym_RBRACK,
    STATE(51), 1,
      aux_sym_tla_record_item_comma_list_repeat1,
  [1142] = 2,
    ACTIONS(159), 1,
      sym_tla_variable,
    STATE(57), 1,
      sym_tla_variable_value,
  [1149] = 1,
    ACTIONS(161), 2,
      ts_builtin_sym_end,
      sym_tla_conjunction,
  [1154] = 2,
    ACTIONS(116), 1,
      sym_tla_variable,
    STATE(61), 1,
      sym_tla_record_field,
  [1161] = 1,
    ACTIONS(163), 2,
      ts_builtin_sym_end,
      sym_tla_conjunction,
  [1166] = 1,
    ACTIONS(165), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1171] = 1,
    ACTIONS(144), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1176] = 1,
    ACTIONS(167), 2,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
  [1181] = 1,
    ACTIONS(131), 2,
      anon_sym_RPAREN,
      anon_sym_AT_AT,
  [1186] = 1,
    ACTIONS(169), 1,
      anon_sym_EQ,
  [1190] = 1,
    ACTIONS(171), 1,
      sym_tla_map_to,
  [1194] = 1,
    ACTIONS(173), 1,
      ts_builtin_sym_end,
  [1198] = 1,
    ACTIONS(175), 1,
      aux_sym_tla_escape_char_token1,
  [1202] = 1,
    ACTIONS(177), 1,
      anon_sym_RBRACK,
  [1206] = 1,
    ACTIONS(179), 1,
      sym_tla_all_map_to,
  [1210] = 1,
    ACTIONS(181), 1,
      anon_sym_RPAREN,
  [1214] = 1,
    ACTIONS(183), 1,
      aux_sym_tla_hex_number_token1,
  [1218] = 1,
    ACTIONS(185), 1,
      anon_sym_RBRACE,
  [1222] = 1,
    ACTIONS(187), 1,
      sym_tla_rangle_bracket,
  [1226] = 1,
    ACTIONS(189), 1,
      aux_sym_tla_binary_number_token1,
  [1230] = 1,
    ACTIONS(191), 1,
      aux_sym_tla_octal_number_token1,
};

static const uint64_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 83,
  [SMALL_STATE(4)] = 163,
  [SMALL_STATE(5)] = 243,
  [SMALL_STATE(6)] = 320,
  [SMALL_STATE(7)] = 394,
  [SMALL_STATE(8)] = 468,
  [SMALL_STATE(9)] = 542,
  [SMALL_STATE(10)] = 616,
  [SMALL_STATE(11)] = 628,
  [SMALL_STATE(12)] = 640,
  [SMALL_STATE(13)] = 652,
  [SMALL_STATE(14)] = 664,
  [SMALL_STATE(15)] = 676,
  [SMALL_STATE(16)] = 688,
  [SMALL_STATE(17)] = 700,
  [SMALL_STATE(18)] = 712,
  [SMALL_STATE(19)] = 724,
  [SMALL_STATE(20)] = 736,
  [SMALL_STATE(21)] = 748,
  [SMALL_STATE(22)] = 760,
  [SMALL_STATE(23)] = 772,
  [SMALL_STATE(24)] = 784,
  [SMALL_STATE(25)] = 796,
  [SMALL_STATE(26)] = 808,
  [SMALL_STATE(27)] = 820,
  [SMALL_STATE(28)] = 832,
  [SMALL_STATE(29)] = 844,
  [SMALL_STATE(30)] = 856,
  [SMALL_STATE(31)] = 868,
  [SMALL_STATE(32)] = 880,
  [SMALL_STATE(33)] = 892,
  [SMALL_STATE(34)] = 904,
  [SMALL_STATE(35)] = 916,
  [SMALL_STATE(36)] = 928,
  [SMALL_STATE(37)] = 940,
  [SMALL_STATE(38)] = 954,
  [SMALL_STATE(39)] = 968,
  [SMALL_STATE(40)] = 982,
  [SMALL_STATE(41)] = 993,
  [SMALL_STATE(42)] = 1004,
  [SMALL_STATE(43)] = 1017,
  [SMALL_STATE(44)] = 1028,
  [SMALL_STATE(45)] = 1038,
  [SMALL_STATE(46)] = 1048,
  [SMALL_STATE(47)] = 1054,
  [SMALL_STATE(48)] = 1064,
  [SMALL_STATE(49)] = 1074,
  [SMALL_STATE(50)] = 1084,
  [SMALL_STATE(51)] = 1094,
  [SMALL_STATE(52)] = 1104,
  [SMALL_STATE(53)] = 1114,
  [SMALL_STATE(54)] = 1122,
  [SMALL_STATE(55)] = 1132,
  [SMALL_STATE(56)] = 1142,
  [SMALL_STATE(57)] = 1149,
  [SMALL_STATE(58)] = 1154,
  [SMALL_STATE(59)] = 1161,
  [SMALL_STATE(60)] = 1166,
  [SMALL_STATE(61)] = 1171,
  [SMALL_STATE(62)] = 1176,
  [SMALL_STATE(63)] = 1181,
  [SMALL_STATE(64)] = 1186,
  [SMALL_STATE(65)] = 1190,
  [SMALL_STATE(66)] = 1194,
  [SMALL_STATE(67)] = 1198,
  [SMALL_STATE(68)] = 1202,
  [SMALL_STATE(69)] = 1206,
  [SMALL_STATE(70)] = 1210,
  [SMALL_STATE(71)] = 1214,
  [SMALL_STATE(72)] = 1218,
  [SMALL_STATE(73)] = 1222,
  [SMALL_STATE(74)] = 1226,
  [SMALL_STATE(75)] = 1230,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_value_expr, 1, .production_id = 4),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_value_expr, 1, .production_id = 11),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_string, 2),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_octal_number, 2),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_finite_set, 2),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_boolean, 1),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_number, 1, .production_id = 2),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_number, 1, .production_id = 3),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_hex_number, 2),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_sequence, 3, .production_id = 19),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_sequence, 2),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_value_expr, 1, .production_id = 5),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_string, 3),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_value_expr, 1, .production_id = 7),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_value_expr, 1, .production_id = 8),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_value_expr, 1, .production_id = 9),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_value_expr, 1, .production_id = 10),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_binary_number, 2),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_value_expr, 1, .production_id = 12),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_number, 1, .production_id = 13),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_number, 1, .production_id = 14),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_number, 1, .production_id = 15),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_record, 2),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_finite_set, 3, .production_id = 18),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_record, 3, .production_id = 16),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_function, 3, .production_id = 17),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_function, 2),
  [89] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_tla_string_repeat1, 2), SHIFT_REPEAT(37),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_tla_string_repeat1, 2),
  [94] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_tla_string_repeat1, 2), SHIFT_REPEAT(67),
  [97] = {.entry = {.count = 1, .reusable = false}}, SHIFT(39),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [103] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_state_conjunction_variable_value_list, 1),
  [109] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_tla_finite_set_item_comma_list_repeat1, 2), SHIFT_REPEAT(6),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_tla_finite_set_item_comma_list_repeat1, 2),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [116] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [118] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_tla_state_conjunction_variable_value_list_repeat1, 2),
  [120] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_tla_state_conjunction_variable_value_list_repeat1, 2), SHIFT_REPEAT(56),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_function_comma_list, 2),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_record_item_comma_list, 1),
  [129] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_tla_function_comma_list_repeat1, 2),
  [133] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_tla_function_comma_list_repeat1, 2), SHIFT_REPEAT(5),
  [136] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_finite_set_item_comma_list, 1),
  [140] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_function_comma_list, 1),
  [142] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_sequence_item_comma_list, 1),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_tla_record_item_comma_list_repeat1, 2),
  [146] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_tla_record_item_comma_list_repeat1, 2), SHIFT_REPEAT(58),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_sequence_item_comma_list, 2),
  [151] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tla_escape_char, 2),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_escape_char, 2),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_finite_set_item_comma_list, 2),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_record_item_comma_list, 2),
  [159] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_conjunction_variable_value, 2, .production_id = 1),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_variable_value, 3, .production_id = 6),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_record_field, 3, .production_id = 20),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tla_mapping, 3, .production_id = 21),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [173] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [177] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [185] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_label_tla(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
