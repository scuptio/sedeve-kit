#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 113
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 77
#define ALIAS_COUNT 0
#define TOKEN_COUNT 37
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 33
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 30

enum ts_symbol_identifiers {
  anon_sym_input = 1,
  anon_sym_output = 2,
  anon_sym_internal = 3,
  anon_sym_struct = 4,
  anon_sym_enum = 5,
  anon_sym_unknown = 6,
  anon_sym_message = 7,
  anon_sym_from = 8,
  anon_sym_to = 9,
  anon_sym_automata = 10,
  anon_sym_Sequence = 11,
  anon_sym_Set = 12,
  anon_sym_NodeId = 13,
  anon_sym_bool = 14,
  anon_sym_i8 = 15,
  anon_sym_i16 = 16,
  anon_sym_i32 = 17,
  anon_sym_i64 = 18,
  anon_sym_u8 = 19,
  anon_sym_u16 = 20,
  anon_sym_u32 = 21,
  anon_sym_u64 = 22,
  anon_sym_f32 = 23,
  anon_sym_f64 = 24,
  anon_sym_string = 25,
  anon_sym_LBRACE = 26,
  anon_sym_RBRACE = 27,
  anon_sym_COMMA = 28,
  anon_sym_COLON = 29,
  anon_sym_LPAREN = 30,
  anon_sym_RPAREN = 31,
  anon_sym_LT = 32,
  anon_sym_GT = 33,
  anon_sym_MsgTrait = 34,
  anon_sym_DASH_GT = 35,
  sym_identifier = 36,
  sym_source_file = 37,
  sym_keyword_primitive_type = 38,
  sym_automaton_or_type_definition = 39,
  sym_struct_definition = 40,
  sym_enum_definition = 41,
  sym_struct_field_list = 42,
  sym_struct_field = 43,
  sym_enum_value_definition_list = 44,
  sym_enum_value_definition = 45,
  sym_enum_payload = 46,
  sym_data_type = 47,
  sym_user_define_type = 48,
  sym_internal_type = 49,
  sym_sequence_type = 50,
  sym_set_type = 51,
  sym_message_type = 52,
  sym_automaton_definition = 53,
  sym_action_definition_list = 54,
  sym_action_type = 55,
  sym_action_definition = 56,
  sym_handle_return = 57,
  sym_parameter_list = 58,
  sym_message_parameter = 59,
  sym_from_parameter = 60,
  sym_to_parameter = 61,
  sym_other_parameter_list = 62,
  sym_parameter = 63,
  sym_struct_name = 64,
  sym_enum_name = 65,
  sym_enum_value_name = 66,
  sym_parameter_name = 67,
  sym_action_handle_name = 68,
  sym_field_name = 69,
  sym_automation_name = 70,
  sym_type_name = 71,
  aux_sym_source_file_repeat1 = 72,
  aux_sym_struct_field_list_repeat1 = 73,
  aux_sym_enum_value_definition_list_repeat1 = 74,
  aux_sym_action_definition_list_repeat1 = 75,
  aux_sym_other_parameter_list_repeat1 = 76,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_input] = "input",
  [anon_sym_output] = "output",
  [anon_sym_internal] = "internal",
  [anon_sym_struct] = "struct",
  [anon_sym_enum] = "enum",
  [anon_sym_unknown] = "unknown",
  [anon_sym_message] = "message",
  [anon_sym_from] = "from",
  [anon_sym_to] = "to",
  [anon_sym_automata] = "automata",
  [anon_sym_Sequence] = "Sequence",
  [anon_sym_Set] = "Set",
  [anon_sym_NodeId] = "NodeId",
  [anon_sym_bool] = "bool",
  [anon_sym_i8] = "i8",
  [anon_sym_i16] = "i16",
  [anon_sym_i32] = "i32",
  [anon_sym_i64] = "i64",
  [anon_sym_u8] = "u8",
  [anon_sym_u16] = "u16",
  [anon_sym_u32] = "u32",
  [anon_sym_u64] = "u64",
  [anon_sym_f32] = "f32",
  [anon_sym_f64] = "f64",
  [anon_sym_string] = "string",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_COMMA] = ",",
  [anon_sym_COLON] = ":",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [anon_sym_MsgTrait] = "MsgTrait",
  [anon_sym_DASH_GT] = "->",
  [sym_identifier] = "identifier",
  [sym_source_file] = "source_file",
  [sym_keyword_primitive_type] = "keyword_primitive_type",
  [sym_automaton_or_type_definition] = "automaton_or_type_definition",
  [sym_struct_definition] = "struct_definition",
  [sym_enum_definition] = "enum_definition",
  [sym_struct_field_list] = "struct_field_list",
  [sym_struct_field] = "struct_field",
  [sym_enum_value_definition_list] = "enum_value_definition_list",
  [sym_enum_value_definition] = "enum_value_definition",
  [sym_enum_payload] = "enum_payload",
  [sym_data_type] = "data_type",
  [sym_user_define_type] = "user_define_type",
  [sym_internal_type] = "internal_type",
  [sym_sequence_type] = "sequence_type",
  [sym_set_type] = "set_type",
  [sym_message_type] = "message_type",
  [sym_automaton_definition] = "automaton_definition",
  [sym_action_definition_list] = "action_definition_list",
  [sym_action_type] = "action_type",
  [sym_action_definition] = "action_definition",
  [sym_handle_return] = "handle_return",
  [sym_parameter_list] = "parameter_list",
  [sym_message_parameter] = "message_parameter",
  [sym_from_parameter] = "from_parameter",
  [sym_to_parameter] = "to_parameter",
  [sym_other_parameter_list] = "other_parameter_list",
  [sym_parameter] = "parameter",
  [sym_struct_name] = "struct_name",
  [sym_enum_name] = "enum_name",
  [sym_enum_value_name] = "enum_value_name",
  [sym_parameter_name] = "parameter_name",
  [sym_action_handle_name] = "action_handle_name",
  [sym_field_name] = "field_name",
  [sym_automation_name] = "automation_name",
  [sym_type_name] = "type_name",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_struct_field_list_repeat1] = "struct_field_list_repeat1",
  [aux_sym_enum_value_definition_list_repeat1] = "enum_value_definition_list_repeat1",
  [aux_sym_action_definition_list_repeat1] = "action_definition_list_repeat1",
  [aux_sym_other_parameter_list_repeat1] = "other_parameter_list_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_input] = anon_sym_input,
  [anon_sym_output] = anon_sym_output,
  [anon_sym_internal] = anon_sym_internal,
  [anon_sym_struct] = anon_sym_struct,
  [anon_sym_enum] = anon_sym_enum,
  [anon_sym_unknown] = anon_sym_unknown,
  [anon_sym_message] = anon_sym_message,
  [anon_sym_from] = anon_sym_from,
  [anon_sym_to] = anon_sym_to,
  [anon_sym_automata] = anon_sym_automata,
  [anon_sym_Sequence] = anon_sym_Sequence,
  [anon_sym_Set] = anon_sym_Set,
  [anon_sym_NodeId] = anon_sym_NodeId,
  [anon_sym_bool] = anon_sym_bool,
  [anon_sym_i8] = anon_sym_i8,
  [anon_sym_i16] = anon_sym_i16,
  [anon_sym_i32] = anon_sym_i32,
  [anon_sym_i64] = anon_sym_i64,
  [anon_sym_u8] = anon_sym_u8,
  [anon_sym_u16] = anon_sym_u16,
  [anon_sym_u32] = anon_sym_u32,
  [anon_sym_u64] = anon_sym_u64,
  [anon_sym_f32] = anon_sym_f32,
  [anon_sym_f64] = anon_sym_f64,
  [anon_sym_string] = anon_sym_string,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_MsgTrait] = anon_sym_MsgTrait,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [sym_identifier] = sym_identifier,
  [sym_source_file] = sym_source_file,
  [sym_keyword_primitive_type] = sym_keyword_primitive_type,
  [sym_automaton_or_type_definition] = sym_automaton_or_type_definition,
  [sym_struct_definition] = sym_struct_definition,
  [sym_enum_definition] = sym_enum_definition,
  [sym_struct_field_list] = sym_struct_field_list,
  [sym_struct_field] = sym_struct_field,
  [sym_enum_value_definition_list] = sym_enum_value_definition_list,
  [sym_enum_value_definition] = sym_enum_value_definition,
  [sym_enum_payload] = sym_enum_payload,
  [sym_data_type] = sym_data_type,
  [sym_user_define_type] = sym_user_define_type,
  [sym_internal_type] = sym_internal_type,
  [sym_sequence_type] = sym_sequence_type,
  [sym_set_type] = sym_set_type,
  [sym_message_type] = sym_message_type,
  [sym_automaton_definition] = sym_automaton_definition,
  [sym_action_definition_list] = sym_action_definition_list,
  [sym_action_type] = sym_action_type,
  [sym_action_definition] = sym_action_definition,
  [sym_handle_return] = sym_handle_return,
  [sym_parameter_list] = sym_parameter_list,
  [sym_message_parameter] = sym_message_parameter,
  [sym_from_parameter] = sym_from_parameter,
  [sym_to_parameter] = sym_to_parameter,
  [sym_other_parameter_list] = sym_other_parameter_list,
  [sym_parameter] = sym_parameter,
  [sym_struct_name] = sym_struct_name,
  [sym_enum_name] = sym_enum_name,
  [sym_enum_value_name] = sym_enum_value_name,
  [sym_parameter_name] = sym_parameter_name,
  [sym_action_handle_name] = sym_action_handle_name,
  [sym_field_name] = sym_field_name,
  [sym_automation_name] = sym_automation_name,
  [sym_type_name] = sym_type_name,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_struct_field_list_repeat1] = aux_sym_struct_field_list_repeat1,
  [aux_sym_enum_value_definition_list_repeat1] = aux_sym_enum_value_definition_list_repeat1,
  [aux_sym_action_definition_list_repeat1] = aux_sym_action_definition_list_repeat1,
  [aux_sym_other_parameter_list_repeat1] = aux_sym_other_parameter_list_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_input] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_output] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_internal] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_struct] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enum] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_unknown] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_message] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_from] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_to] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_automata] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Sequence] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Set] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_NodeId] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bool] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_i8] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_i16] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_i32] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_i64] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_u8] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_u16] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_u32] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_u64] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_f32] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_f64] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_string] = {
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
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
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
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_MsgTrait] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_keyword_primitive_type] = {
    .visible = true,
    .named = true,
  },
  [sym_automaton_or_type_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_field_list] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_field] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_value_definition_list] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_value_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_payload] = {
    .visible = true,
    .named = true,
  },
  [sym_data_type] = {
    .visible = true,
    .named = true,
  },
  [sym_user_define_type] = {
    .visible = true,
    .named = true,
  },
  [sym_internal_type] = {
    .visible = true,
    .named = true,
  },
  [sym_sequence_type] = {
    .visible = true,
    .named = true,
  },
  [sym_set_type] = {
    .visible = true,
    .named = true,
  },
  [sym_message_type] = {
    .visible = true,
    .named = true,
  },
  [sym_automaton_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_action_definition_list] = {
    .visible = true,
    .named = true,
  },
  [sym_action_type] = {
    .visible = true,
    .named = true,
  },
  [sym_action_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_handle_return] = {
    .visible = true,
    .named = true,
  },
  [sym_parameter_list] = {
    .visible = true,
    .named = true,
  },
  [sym_message_parameter] = {
    .visible = true,
    .named = true,
  },
  [sym_from_parameter] = {
    .visible = true,
    .named = true,
  },
  [sym_to_parameter] = {
    .visible = true,
    .named = true,
  },
  [sym_other_parameter_list] = {
    .visible = true,
    .named = true,
  },
  [sym_parameter] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_name] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_name] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_value_name] = {
    .visible = true,
    .named = true,
  },
  [sym_parameter_name] = {
    .visible = true,
    .named = true,
  },
  [sym_action_handle_name] = {
    .visible = true,
    .named = true,
  },
  [sym_field_name] = {
    .visible = true,
    .named = true,
  },
  [sym_automation_name] = {
    .visible = true,
    .named = true,
  },
  [sym_type_name] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_struct_field_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_enum_value_definition_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_action_definition_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_other_parameter_list_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_action_definition_list = 1,
  field_action_handle_name = 2,
  field_action_type = 3,
  field_automation_name = 4,
  field_automaton_definition = 5,
  field_data_type = 6,
  field_data_type_message = 7,
  field_data_type_name = 8,
  field_data_type_node_id = 9,
  field_data_type_sequence = 10,
  field_data_type_set = 11,
  field_data_type_unknown = 12,
  field_enum_definition = 13,
  field_enum_name = 14,
  field_enum_payload = 15,
  field_enum_value_definition_list = 16,
  field_enum_value_name = 17,
  field_field_name = 18,
  field_from_parameter = 19,
  field_internal_type = 20,
  field_message_parameter = 21,
  field_other_parameter_list = 22,
  field_parameter_list = 23,
  field_parameter_name = 24,
  field_parameter_type = 25,
  field_primitive_type = 26,
  field_return_value = 27,
  field_struct_definition = 28,
  field_struct_field_list = 29,
  field_struct_name = 30,
  field_to_parameter = 31,
  field_type_node_id = 32,
  field_user_define_type = 33,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_action_definition_list] = "action_definition_list",
  [field_action_handle_name] = "action_handle_name",
  [field_action_type] = "action_type",
  [field_automation_name] = "automation_name",
  [field_automaton_definition] = "automaton_definition",
  [field_data_type] = "data_type",
  [field_data_type_message] = "data_type_message",
  [field_data_type_name] = "data_type_name",
  [field_data_type_node_id] = "data_type_node_id",
  [field_data_type_sequence] = "data_type_sequence",
  [field_data_type_set] = "data_type_set",
  [field_data_type_unknown] = "data_type_unknown",
  [field_enum_definition] = "enum_definition",
  [field_enum_name] = "enum_name",
  [field_enum_payload] = "enum_payload",
  [field_enum_value_definition_list] = "enum_value_definition_list",
  [field_enum_value_name] = "enum_value_name",
  [field_field_name] = "field_name",
  [field_from_parameter] = "from_parameter",
  [field_internal_type] = "internal_type",
  [field_message_parameter] = "message_parameter",
  [field_other_parameter_list] = "other_parameter_list",
  [field_parameter_list] = "parameter_list",
  [field_parameter_name] = "parameter_name",
  [field_parameter_type] = "parameter_type",
  [field_primitive_type] = "primitive_type",
  [field_return_value] = "return_value",
  [field_struct_definition] = "struct_definition",
  [field_struct_field_list] = "struct_field_list",
  [field_struct_name] = "struct_name",
  [field_to_parameter] = "to_parameter",
  [field_type_node_id] = "type_node_id",
  [field_user_define_type] = "user_define_type",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 1},
  [5] = {.index = 4, .length = 2},
  [6] = {.index = 6, .length = 2},
  [7] = {.index = 8, .length = 2},
  [8] = {.index = 10, .length = 2},
  [9] = {.index = 12, .length = 1},
  [10] = {.index = 13, .length = 1},
  [11] = {.index = 14, .length = 2},
  [12] = {.index = 16, .length = 1},
  [13] = {.index = 17, .length = 1},
  [14] = {.index = 18, .length = 1},
  [15] = {.index = 19, .length = 1},
  [16] = {.index = 20, .length = 1},
  [17] = {.index = 21, .length = 1},
  [18] = {.index = 22, .length = 2},
  [19] = {.index = 24, .length = 1},
  [20] = {.index = 25, .length = 1},
  [21] = {.index = 26, .length = 3},
  [22] = {.index = 29, .length = 1},
  [23] = {.index = 30, .length = 2},
  [24] = {.index = 32, .length = 4},
  [25] = {.index = 36, .length = 2},
  [26] = {.index = 38, .length = 3},
  [27] = {.index = 41, .length = 2},
  [28] = {.index = 43, .length = 2},
  [29] = {.index = 45, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_struct_definition, 0},
  [1] =
    {field_enum_definition, 0},
  [2] =
    {field_automaton_definition, 0},
  [3] =
    {field_enum_value_name, 0},
  [4] =
    {field_struct_field_list, 3},
    {field_struct_name, 1},
  [6] =
    {field_enum_name, 1},
    {field_enum_value_definition_list, 3},
  [8] =
    {field_enum_payload, 1},
    {field_enum_value_name, 0},
  [10] =
    {field_action_definition_list, 3},
    {field_automation_name, 1},
  [12] =
    {field_data_type_node_id, 0},
  [13] =
    {field_primitive_type, 0},
  [14] =
    {field_data_type, 2},
    {field_field_name, 0},
  [16] =
    {field_user_define_type, 0},
  [17] =
    {field_internal_type, 0},
  [18] =
    {field_data_type_sequence, 0},
  [19] =
    {field_data_type_set, 0},
  [20] =
    {field_data_type_message, 0},
  [21] =
    {field_data_type_name, 0},
  [22] =
    {field_data_type_name, 1},
    {field_data_type_unknown, 0},
  [24] =
    {field_data_type, 1},
  [25] =
    {field_message_parameter, 0},
  [26] =
    {field_action_handle_name, 1},
    {field_action_type, 0},
    {field_parameter_list, 3},
  [29] =
    {field_data_type, 2},
  [30] =
    {field_parameter_name, 0},
    {field_user_define_type, 2},
  [32] =
    {field_action_handle_name, 1},
    {field_action_type, 0},
    {field_parameter_list, 3},
    {field_return_value, 5},
  [36] =
    {field_message_parameter, 0},
    {field_other_parameter_list, 2},
  [38] =
    {field_from_parameter, 2},
    {field_message_parameter, 0},
    {field_to_parameter, 3},
  [41] =
    {field_parameter_name, 0},
    {field_type_node_id, 2},
  [43] =
    {field_parameter_name, 0},
    {field_parameter_type, 2},
  [45] =
    {field_from_parameter, 2},
    {field_message_parameter, 0},
    {field_other_parameter_list, 5},
    {field_to_parameter, 3},
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
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  switch (state) {
    case 0:
      if (eof) ADVANCE(83);
      if (lookahead == '(') ADVANCE(130);
      if (lookahead == ')') ADVANCE(131);
      if (lookahead == ',') ADVANCE(128);
      if (lookahead == '-') ADVANCE(13);
      if (lookahead == ':') ADVANCE(129);
      if (lookahead == '<') ADVANCE(132);
      if (lookahead == '>') ADVANCE(133);
      if (lookahead == 'M') ADVANCE(64);
      if (lookahead == 'N') ADVANCE(51);
      if (lookahead == 'S') ADVANCE(26);
      if (lookahead == 'a') ADVANCE(76);
      if (lookahead == 'b') ADVANCE(57);
      if (lookahead == 'e') ADVANCE(46);
      if (lookahead == 'f') ADVANCE(7);
      if (lookahead == 'i') ADVANCE(2);
      if (lookahead == 'm') ADVANCE(30);
      if (lookahead == 'o') ADVANCE(77);
      if (lookahead == 's') ADVANCE(67);
      if (lookahead == 't') ADVANCE(52);
      if (lookahead == 'u') ADVANCE(3);
      if (lookahead == '{') ADVANCE(126);
      if (lookahead == '}') ADVANCE(127);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == ')') ADVANCE(131);
      if (lookahead == '}') ADVANCE(127);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(1)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 2:
      if (lookahead == '1') ADVANCE(11);
      if (lookahead == '3') ADVANCE(5);
      if (lookahead == '6') ADVANCE(9);
      if (lookahead == '8') ADVANCE(104);
      if (lookahead == 'n') ADVANCE(58);
      END_STATE();
    case 3:
      if (lookahead == '1') ADVANCE(12);
      if (lookahead == '3') ADVANCE(6);
      if (lookahead == '6') ADVANCE(10);
      if (lookahead == '8') ADVANCE(112);
      if (lookahead == 'n') ADVANCE(39);
      END_STATE();
    case 4:
      if (lookahead == '2') ADVANCE(120);
      END_STATE();
    case 5:
      if (lookahead == '2') ADVANCE(108);
      END_STATE();
    case 6:
      if (lookahead == '2') ADVANCE(116);
      END_STATE();
    case 7:
      if (lookahead == '3') ADVANCE(4);
      if (lookahead == '6') ADVANCE(8);
      if (lookahead == 'r') ADVANCE(55);
      END_STATE();
    case 8:
      if (lookahead == '4') ADVANCE(122);
      END_STATE();
    case 9:
      if (lookahead == '4') ADVANCE(110);
      END_STATE();
    case 10:
      if (lookahead == '4') ADVANCE(118);
      END_STATE();
    case 11:
      if (lookahead == '6') ADVANCE(106);
      END_STATE();
    case 12:
      if (lookahead == '6') ADVANCE(114);
      END_STATE();
    case 13:
      if (lookahead == '>') ADVANCE(136);
      END_STATE();
    case 14:
      if (lookahead == 'I') ADVANCE(24);
      END_STATE();
    case 15:
      if (lookahead == 'M') ADVANCE(179);
      if (lookahead == 'N') ADVANCE(170);
      if (lookahead == 'S') ADVANCE(154);
      if (lookahead == 'b') ADVANCE(174);
      if (lookahead == 'f') ADVANCE(142);
      if (lookahead == 'i') ADVANCE(137);
      if (lookahead == 's') ADVANCE(180);
      if (lookahead == 'u') ADVANCE(138);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(15)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 16:
      if (lookahead == 'T') ADVANCE(62);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(38);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(95);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(36);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(41);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(73);
      END_STATE();
    case 22:
      if (lookahead == 'c') ADVANCE(71);
      END_STATE();
    case 23:
      if (lookahead == 'c') ADVANCE(29);
      END_STATE();
    case 24:
      if (lookahead == 'd') ADVANCE(100);
      END_STATE();
    case 25:
      if (lookahead == 'd') ADVANCE(27);
      END_STATE();
    case 26:
      if (lookahead == 'e') ADVANCE(60);
      END_STATE();
    case 27:
      if (lookahead == 'e') ADVANCE(14);
      END_STATE();
    case 28:
      if (lookahead == 'e') ADVANCE(91);
      END_STATE();
    case 29:
      if (lookahead == 'e') ADVANCE(96);
      END_STATE();
    case 30:
      if (lookahead == 'e') ADVANCE(66);
      END_STATE();
    case 31:
      if (lookahead == 'e') ADVANCE(63);
      END_STATE();
    case 32:
      if (lookahead == 'e') ADVANCE(47);
      END_STATE();
    case 33:
      if (lookahead == 'f') ADVANCE(178);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(33)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 34:
      if (lookahead == 'g') ADVANCE(16);
      END_STATE();
    case 35:
      if (lookahead == 'g') ADVANCE(124);
      END_STATE();
    case 36:
      if (lookahead == 'g') ADVANCE(28);
      END_STATE();
    case 37:
      if (lookahead == 'i') ADVANCE(48);
      if (lookahead == 'u') ADVANCE(22);
      END_STATE();
    case 38:
      if (lookahead == 'i') ADVANCE(72);
      END_STATE();
    case 39:
      if (lookahead == 'k') ADVANCE(50);
      END_STATE();
    case 40:
      if (lookahead == 'l') ADVANCE(102);
      END_STATE();
    case 41:
      if (lookahead == 'l') ADVANCE(86);
      END_STATE();
    case 42:
      if (lookahead == 'm') ADVANCE(88);
      END_STATE();
    case 43:
      if (lookahead == 'm') ADVANCE(92);
      END_STATE();
    case 44:
      if (lookahead == 'm') ADVANCE(21);
      END_STATE();
    case 45:
      if (lookahead == 'n') ADVANCE(89);
      END_STATE();
    case 46:
      if (lookahead == 'n') ADVANCE(75);
      END_STATE();
    case 47:
      if (lookahead == 'n') ADVANCE(23);
      END_STATE();
    case 48:
      if (lookahead == 'n') ADVANCE(35);
      END_STATE();
    case 49:
      if (lookahead == 'n') ADVANCE(20);
      END_STATE();
    case 50:
      if (lookahead == 'n') ADVANCE(54);
      END_STATE();
    case 51:
      if (lookahead == 'o') ADVANCE(25);
      END_STATE();
    case 52:
      if (lookahead == 'o') ADVANCE(94);
      END_STATE();
    case 53:
      if (lookahead == 'o') ADVANCE(40);
      END_STATE();
    case 54:
      if (lookahead == 'o') ADVANCE(82);
      END_STATE();
    case 55:
      if (lookahead == 'o') ADVANCE(43);
      END_STATE();
    case 56:
      if (lookahead == 'o') ADVANCE(44);
      END_STATE();
    case 57:
      if (lookahead == 'o') ADVANCE(53);
      END_STATE();
    case 58:
      if (lookahead == 'p') ADVANCE(78);
      if (lookahead == 't') ADVANCE(31);
      END_STATE();
    case 59:
      if (lookahead == 'p') ADVANCE(80);
      END_STATE();
    case 60:
      if (lookahead == 'q') ADVANCE(79);
      if (lookahead == 't') ADVANCE(98);
      END_STATE();
    case 61:
      if (lookahead == 'r') ADVANCE(37);
      END_STATE();
    case 62:
      if (lookahead == 'r') ADVANCE(17);
      END_STATE();
    case 63:
      if (lookahead == 'r') ADVANCE(49);
      END_STATE();
    case 64:
      if (lookahead == 's') ADVANCE(34);
      END_STATE();
    case 65:
      if (lookahead == 's') ADVANCE(19);
      END_STATE();
    case 66:
      if (lookahead == 's') ADVANCE(65);
      END_STATE();
    case 67:
      if (lookahead == 't') ADVANCE(61);
      END_STATE();
    case 68:
      if (lookahead == 't') ADVANCE(59);
      END_STATE();
    case 69:
      if (lookahead == 't') ADVANCE(84);
      END_STATE();
    case 70:
      if (lookahead == 't') ADVANCE(85);
      END_STATE();
    case 71:
      if (lookahead == 't') ADVANCE(87);
      END_STATE();
    case 72:
      if (lookahead == 't') ADVANCE(134);
      END_STATE();
    case 73:
      if (lookahead == 't') ADVANCE(18);
      END_STATE();
    case 74:
      if (lookahead == 't') ADVANCE(56);
      END_STATE();
    case 75:
      if (lookahead == 'u') ADVANCE(42);
      END_STATE();
    case 76:
      if (lookahead == 'u') ADVANCE(74);
      END_STATE();
    case 77:
      if (lookahead == 'u') ADVANCE(68);
      END_STATE();
    case 78:
      if (lookahead == 'u') ADVANCE(69);
      END_STATE();
    case 79:
      if (lookahead == 'u') ADVANCE(32);
      END_STATE();
    case 80:
      if (lookahead == 'u') ADVANCE(70);
      END_STATE();
    case 81:
      if (lookahead == 'u') ADVANCE(165);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(81)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 82:
      if (lookahead == 'w') ADVANCE(45);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_input);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(anon_sym_internal);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(anon_sym_struct);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(anon_sym_enum);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(anon_sym_unknown);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_unknown);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_message);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_from);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_from);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_to);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_automata);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_Sequence);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_Sequence);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_Set);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(anon_sym_Set);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_NodeId);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_NodeId);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_bool);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_bool);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_i8);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_i8);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_i16);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_i16);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(anon_sym_i32);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_i32);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_i64);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_i64);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_u8);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(anon_sym_u8);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(anon_sym_u16);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(anon_sym_u16);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(anon_sym_u32);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(anon_sym_u32);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(anon_sym_u64);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(anon_sym_u64);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(anon_sym_f32);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(anon_sym_f32);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(anon_sym_f64);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(anon_sym_f64);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(anon_sym_string);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(anon_sym_string);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(anon_sym_LT);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(anon_sym_MsgTrait);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(anon_sym_MsgTrait);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 136:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '1') ADVANCE(146);
      if (lookahead == '3') ADVANCE(140);
      if (lookahead == '6') ADVANCE(144);
      if (lookahead == '8') ADVANCE(105);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '1') ADVANCE(147);
      if (lookahead == '3') ADVANCE(141);
      if (lookahead == '6') ADVANCE(145);
      if (lookahead == '8') ADVANCE(113);
      if (lookahead == 'n') ADVANCE(162);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 139:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '2') ADVANCE(121);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 140:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '2') ADVANCE(109);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '2') ADVANCE(117);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 142:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '3') ADVANCE(139);
      if (lookahead == '6') ADVANCE(143);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 143:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '4') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '4') ADVANCE(111);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '4') ADVANCE(119);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 146:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '6') ADVANCE(107);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '6') ADVANCE(115);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I') ADVANCE(152);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T') ADVANCE(177);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(160);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(156);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'd') ADVANCE(101);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'd') ADVANCE(155);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 154:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(175);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(148);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(97);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(166);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 158:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(149);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(125);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(181);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 161:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(168);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 162:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'k') ADVANCE(169);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(103);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(93);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(162);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(151);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(90);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(159);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(172);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(153);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(163);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(183);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(164);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(171);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'q') ADVANCE(182);
      if (lookahead == 't') ADVANCE(99);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(161);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(150);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(173);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(158);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(176);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(135);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(157);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'w') ADVANCE(167);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 15},
  [3] = {.lex_state = 15},
  [4] = {.lex_state = 15},
  [5] = {.lex_state = 15},
  [6] = {.lex_state = 15},
  [7] = {.lex_state = 15},
  [8] = {.lex_state = 15},
  [9] = {.lex_state = 0},
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
  [28] = {.lex_state = 33},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 1},
  [31] = {.lex_state = 1},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 81},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 1},
  [41] = {.lex_state = 1},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 1},
  [45] = {.lex_state = 1},
  [46] = {.lex_state = 1},
  [47] = {.lex_state = 1},
  [48] = {.lex_state = 1},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 1},
  [52] = {.lex_state = 1},
  [53] = {.lex_state = 0},
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 0},
  [56] = {.lex_state = 1},
  [57] = {.lex_state = 0},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 0},
  [63] = {.lex_state = 0},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 0},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 1},
  [71] = {.lex_state = 1},
  [72] = {.lex_state = 1},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 0},
  [77] = {.lex_state = 0},
  [78] = {.lex_state = 1},
  [79] = {.lex_state = 1},
  [80] = {.lex_state = 0},
  [81] = {.lex_state = 0},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 0},
  [84] = {.lex_state = 0},
  [85] = {.lex_state = 0},
  [86] = {.lex_state = 0},
  [87] = {.lex_state = 0},
  [88] = {.lex_state = 0},
  [89] = {.lex_state = 0},
  [90] = {.lex_state = 0},
  [91] = {.lex_state = 0},
  [92] = {.lex_state = 0},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 1},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_input] = ACTIONS(1),
    [anon_sym_output] = ACTIONS(1),
    [anon_sym_internal] = ACTIONS(1),
    [anon_sym_struct] = ACTIONS(1),
    [anon_sym_enum] = ACTIONS(1),
    [anon_sym_unknown] = ACTIONS(1),
    [anon_sym_message] = ACTIONS(1),
    [anon_sym_from] = ACTIONS(1),
    [anon_sym_to] = ACTIONS(1),
    [anon_sym_automata] = ACTIONS(1),
    [anon_sym_Sequence] = ACTIONS(1),
    [anon_sym_Set] = ACTIONS(1),
    [anon_sym_NodeId] = ACTIONS(1),
    [anon_sym_bool] = ACTIONS(1),
    [anon_sym_i8] = ACTIONS(1),
    [anon_sym_i16] = ACTIONS(1),
    [anon_sym_i32] = ACTIONS(1),
    [anon_sym_i64] = ACTIONS(1),
    [anon_sym_u8] = ACTIONS(1),
    [anon_sym_u16] = ACTIONS(1),
    [anon_sym_u32] = ACTIONS(1),
    [anon_sym_u64] = ACTIONS(1),
    [anon_sym_f32] = ACTIONS(1),
    [anon_sym_f64] = ACTIONS(1),
    [anon_sym_string] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_MsgTrait] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(85),
    [sym_automaton_or_type_definition] = STATE(9),
    [sym_struct_definition] = STATE(35),
    [sym_enum_definition] = STATE(36),
    [sym_automaton_definition] = STATE(42),
    [aux_sym_source_file_repeat1] = STATE(9),
    [anon_sym_struct] = ACTIONS(3),
    [anon_sym_enum] = ACTIONS(5),
    [anon_sym_automata] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 15,
    ACTIONS(9), 1,
      anon_sym_unknown,
    ACTIONS(11), 1,
      anon_sym_Sequence,
    ACTIONS(13), 1,
      anon_sym_Set,
    ACTIONS(15), 1,
      anon_sym_NodeId,
    ACTIONS(19), 1,
      anon_sym_MsgTrait,
    ACTIONS(21), 1,
      sym_identifier,
    STATE(12), 1,
      sym_message_type,
    STATE(16), 1,
      sym_set_type,
    STATE(17), 1,
      sym_sequence_type,
    STATE(18), 1,
      sym_internal_type,
    STATE(20), 1,
      sym_user_define_type,
    STATE(21), 1,
      sym_keyword_primitive_type,
    STATE(25), 1,
      sym_type_name,
    STATE(68), 1,
      sym_data_type,
    ACTIONS(17), 12,
      anon_sym_bool,
      anon_sym_i8,
      anon_sym_i16,
      anon_sym_i32,
      anon_sym_i64,
      anon_sym_u8,
      anon_sym_u16,
      anon_sym_u32,
      anon_sym_u64,
      anon_sym_f32,
      anon_sym_f64,
      anon_sym_string,
  [57] = 15,
    ACTIONS(9), 1,
      anon_sym_unknown,
    ACTIONS(11), 1,
      anon_sym_Sequence,
    ACTIONS(13), 1,
      anon_sym_Set,
    ACTIONS(15), 1,
      anon_sym_NodeId,
    ACTIONS(19), 1,
      anon_sym_MsgTrait,
    ACTIONS(21), 1,
      sym_identifier,
    STATE(12), 1,
      sym_message_type,
    STATE(16), 1,
      sym_set_type,
    STATE(17), 1,
      sym_sequence_type,
    STATE(18), 1,
      sym_internal_type,
    STATE(20), 1,
      sym_user_define_type,
    STATE(21), 1,
      sym_keyword_primitive_type,
    STATE(25), 1,
      sym_type_name,
    STATE(63), 1,
      sym_data_type,
    ACTIONS(17), 12,
      anon_sym_bool,
      anon_sym_i8,
      anon_sym_i16,
      anon_sym_i32,
      anon_sym_i64,
      anon_sym_u8,
      anon_sym_u16,
      anon_sym_u32,
      anon_sym_u64,
      anon_sym_f32,
      anon_sym_f64,
      anon_sym_string,
  [114] = 15,
    ACTIONS(9), 1,
      anon_sym_unknown,
    ACTIONS(11), 1,
      anon_sym_Sequence,
    ACTIONS(13), 1,
      anon_sym_Set,
    ACTIONS(15), 1,
      anon_sym_NodeId,
    ACTIONS(19), 1,
      anon_sym_MsgTrait,
    ACTIONS(21), 1,
      sym_identifier,
    STATE(12), 1,
      sym_message_type,
    STATE(16), 1,
      sym_set_type,
    STATE(17), 1,
      sym_sequence_type,
    STATE(18), 1,
      sym_internal_type,
    STATE(20), 1,
      sym_user_define_type,
    STATE(21), 1,
      sym_keyword_primitive_type,
    STATE(25), 1,
      sym_type_name,
    STATE(38), 1,
      sym_data_type,
    ACTIONS(17), 12,
      anon_sym_bool,
      anon_sym_i8,
      anon_sym_i16,
      anon_sym_i32,
      anon_sym_i64,
      anon_sym_u8,
      anon_sym_u16,
      anon_sym_u32,
      anon_sym_u64,
      anon_sym_f32,
      anon_sym_f64,
      anon_sym_string,
  [171] = 15,
    ACTIONS(9), 1,
      anon_sym_unknown,
    ACTIONS(11), 1,
      anon_sym_Sequence,
    ACTIONS(13), 1,
      anon_sym_Set,
    ACTIONS(15), 1,
      anon_sym_NodeId,
    ACTIONS(19), 1,
      anon_sym_MsgTrait,
    ACTIONS(21), 1,
      sym_identifier,
    STATE(12), 1,
      sym_message_type,
    STATE(16), 1,
      sym_set_type,
    STATE(17), 1,
      sym_sequence_type,
    STATE(18), 1,
      sym_internal_type,
    STATE(20), 1,
      sym_user_define_type,
    STATE(21), 1,
      sym_keyword_primitive_type,
    STATE(25), 1,
      sym_type_name,
    STATE(92), 1,
      sym_data_type,
    ACTIONS(17), 12,
      anon_sym_bool,
      anon_sym_i8,
      anon_sym_i16,
      anon_sym_i32,
      anon_sym_i64,
      anon_sym_u8,
      anon_sym_u16,
      anon_sym_u32,
      anon_sym_u64,
      anon_sym_f32,
      anon_sym_f64,
      anon_sym_string,
  [228] = 15,
    ACTIONS(9), 1,
      anon_sym_unknown,
    ACTIONS(11), 1,
      anon_sym_Sequence,
    ACTIONS(13), 1,
      anon_sym_Set,
    ACTIONS(15), 1,
      anon_sym_NodeId,
    ACTIONS(19), 1,
      anon_sym_MsgTrait,
    ACTIONS(21), 1,
      sym_identifier,
    STATE(12), 1,
      sym_message_type,
    STATE(16), 1,
      sym_set_type,
    STATE(17), 1,
      sym_sequence_type,
    STATE(18), 1,
      sym_internal_type,
    STATE(20), 1,
      sym_user_define_type,
    STATE(21), 1,
      sym_keyword_primitive_type,
    STATE(25), 1,
      sym_type_name,
    STATE(89), 1,
      sym_data_type,
    ACTIONS(17), 12,
      anon_sym_bool,
      anon_sym_i8,
      anon_sym_i16,
      anon_sym_i32,
      anon_sym_i64,
      anon_sym_u8,
      anon_sym_u16,
      anon_sym_u32,
      anon_sym_u64,
      anon_sym_f32,
      anon_sym_f64,
      anon_sym_string,
  [285] = 15,
    ACTIONS(9), 1,
      anon_sym_unknown,
    ACTIONS(11), 1,
      anon_sym_Sequence,
    ACTIONS(13), 1,
      anon_sym_Set,
    ACTIONS(15), 1,
      anon_sym_NodeId,
    ACTIONS(19), 1,
      anon_sym_MsgTrait,
    ACTIONS(21), 1,
      sym_identifier,
    STATE(12), 1,
      sym_message_type,
    STATE(16), 1,
      sym_set_type,
    STATE(17), 1,
      sym_sequence_type,
    STATE(18), 1,
      sym_internal_type,
    STATE(20), 1,
      sym_user_define_type,
    STATE(21), 1,
      sym_keyword_primitive_type,
    STATE(25), 1,
      sym_type_name,
    STATE(88), 1,
      sym_data_type,
    ACTIONS(17), 12,
      anon_sym_bool,
      anon_sym_i8,
      anon_sym_i16,
      anon_sym_i32,
      anon_sym_i64,
      anon_sym_u8,
      anon_sym_u16,
      anon_sym_u32,
      anon_sym_u64,
      anon_sym_f32,
      anon_sym_f64,
      anon_sym_string,
  [342] = 15,
    ACTIONS(9), 1,
      anon_sym_unknown,
    ACTIONS(11), 1,
      anon_sym_Sequence,
    ACTIONS(13), 1,
      anon_sym_Set,
    ACTIONS(15), 1,
      anon_sym_NodeId,
    ACTIONS(19), 1,
      anon_sym_MsgTrait,
    ACTIONS(21), 1,
      sym_identifier,
    STATE(12), 1,
      sym_message_type,
    STATE(16), 1,
      sym_set_type,
    STATE(17), 1,
      sym_sequence_type,
    STATE(18), 1,
      sym_internal_type,
    STATE(20), 1,
      sym_user_define_type,
    STATE(21), 1,
      sym_keyword_primitive_type,
    STATE(25), 1,
      sym_type_name,
    STATE(99), 1,
      sym_data_type,
    ACTIONS(17), 12,
      anon_sym_bool,
      anon_sym_i8,
      anon_sym_i16,
      anon_sym_i32,
      anon_sym_i64,
      anon_sym_u8,
      anon_sym_u16,
      anon_sym_u32,
      anon_sym_u64,
      anon_sym_f32,
      anon_sym_f64,
      anon_sym_string,
  [399] = 8,
    ACTIONS(3), 1,
      anon_sym_struct,
    ACTIONS(5), 1,
      anon_sym_enum,
    ACTIONS(7), 1,
      anon_sym_automata,
    ACTIONS(23), 1,
      ts_builtin_sym_end,
    STATE(35), 1,
      sym_struct_definition,
    STATE(36), 1,
      sym_enum_definition,
    STATE(42), 1,
      sym_automaton_definition,
    STATE(10), 2,
      sym_automaton_or_type_definition,
      aux_sym_source_file_repeat1,
  [425] = 8,
    ACTIONS(25), 1,
      ts_builtin_sym_end,
    ACTIONS(27), 1,
      anon_sym_struct,
    ACTIONS(30), 1,
      anon_sym_enum,
    ACTIONS(33), 1,
      anon_sym_automata,
    STATE(35), 1,
      sym_struct_definition,
    STATE(36), 1,
      sym_enum_definition,
    STATE(42), 1,
      sym_automaton_definition,
    STATE(10), 2,
      sym_automaton_or_type_definition,
      aux_sym_source_file_repeat1,
  [451] = 1,
    ACTIONS(36), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [461] = 1,
    ACTIONS(38), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [471] = 1,
    ACTIONS(40), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [481] = 1,
    ACTIONS(42), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [491] = 1,
    ACTIONS(44), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [501] = 1,
    ACTIONS(46), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [511] = 1,
    ACTIONS(48), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [521] = 1,
    ACTIONS(50), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [531] = 4,
    STATE(72), 1,
      sym_action_type,
    STATE(112), 1,
      sym_action_definition_list,
    STATE(27), 2,
      sym_action_definition,
      aux_sym_action_definition_list_repeat1,
    ACTIONS(52), 3,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
  [547] = 1,
    ACTIONS(54), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [557] = 1,
    ACTIONS(56), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [567] = 1,
    ACTIONS(58), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [577] = 1,
    ACTIONS(60), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [587] = 1,
    ACTIONS(62), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [597] = 1,
    ACTIONS(64), 7,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [607] = 4,
    ACTIONS(69), 1,
      anon_sym_RBRACE,
    STATE(72), 1,
      sym_action_type,
    STATE(26), 2,
      sym_action_definition,
      aux_sym_action_definition_list_repeat1,
    ACTIONS(66), 3,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
  [623] = 4,
    ACTIONS(71), 1,
      anon_sym_RBRACE,
    STATE(72), 1,
      sym_action_type,
    STATE(26), 2,
      sym_action_definition,
      aux_sym_action_definition_list_repeat1,
    ACTIONS(52), 3,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
  [639] = 6,
    ACTIONS(73), 1,
      anon_sym_from,
    ACTIONS(75), 1,
      sym_identifier,
    STATE(55), 1,
      sym_parameter,
    STATE(69), 1,
      sym_from_parameter,
    STATE(93), 1,
      sym_other_parameter_list,
    STATE(95), 1,
      sym_parameter_name,
  [658] = 3,
    ACTIONS(79), 1,
      anon_sym_DASH_GT,
    STATE(32), 1,
      sym_handle_return,
    ACTIONS(77), 4,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
  [671] = 4,
    ACTIONS(81), 1,
      sym_identifier,
    STATE(33), 1,
      sym_enum_value_name,
    STATE(58), 1,
      sym_enum_value_definition,
    STATE(108), 1,
      sym_enum_value_definition_list,
  [684] = 4,
    ACTIONS(83), 1,
      sym_identifier,
    STATE(55), 1,
      sym_parameter,
    STATE(95), 1,
      sym_parameter_name,
    STATE(111), 1,
      sym_other_parameter_list,
  [697] = 1,
    ACTIONS(85), 4,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
  [704] = 3,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(64), 1,
      sym_enum_payload,
    ACTIONS(87), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [715] = 4,
    ACTIONS(9), 1,
      anon_sym_unknown,
    ACTIONS(21), 1,
      sym_identifier,
    STATE(25), 1,
      sym_type_name,
    STATE(77), 1,
      sym_user_define_type,
  [728] = 1,
    ACTIONS(91), 4,
      ts_builtin_sym_end,
      anon_sym_struct,
      anon_sym_enum,
      anon_sym_automata,
  [735] = 1,
    ACTIONS(93), 4,
      ts_builtin_sym_end,
      anon_sym_struct,
      anon_sym_enum,
      anon_sym_automata,
  [742] = 1,
    ACTIONS(95), 4,
      ts_builtin_sym_end,
      anon_sym_struct,
      anon_sym_enum,
      anon_sym_automata,
  [749] = 1,
    ACTIONS(97), 4,
      anon_sym_input,
      anon_sym_output,
      anon_sym_internal,
      anon_sym_RBRACE,
  [756] = 1,
    ACTIONS(99), 4,
      ts_builtin_sym_end,
      anon_sym_struct,
      anon_sym_enum,
      anon_sym_automata,
  [763] = 4,
    ACTIONS(81), 1,
      sym_identifier,
    ACTIONS(101), 1,
      anon_sym_RBRACE,
    STATE(33), 1,
      sym_enum_value_name,
    STATE(67), 1,
      sym_enum_value_definition,
  [776] = 4,
    ACTIONS(81), 1,
      sym_identifier,
    ACTIONS(103), 1,
      anon_sym_RBRACE,
    STATE(33), 1,
      sym_enum_value_name,
    STATE(67), 1,
      sym_enum_value_definition,
  [789] = 1,
    ACTIONS(105), 4,
      ts_builtin_sym_end,
      anon_sym_struct,
      anon_sym_enum,
      anon_sym_automata,
  [796] = 1,
    ACTIONS(107), 4,
      ts_builtin_sym_end,
      anon_sym_struct,
      anon_sym_enum,
      anon_sym_automata,
  [803] = 4,
    ACTIONS(109), 1,
      anon_sym_RBRACE,
    ACTIONS(111), 1,
      sym_identifier,
    STATE(66), 1,
      sym_struct_field,
    STATE(105), 1,
      sym_field_name,
  [816] = 4,
    ACTIONS(111), 1,
      sym_identifier,
    STATE(49), 1,
      sym_struct_field,
    STATE(102), 1,
      sym_struct_field_list,
    STATE(105), 1,
      sym_field_name,
  [829] = 4,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(113), 1,
      anon_sym_RPAREN,
    STATE(73), 1,
      sym_parameter,
    STATE(95), 1,
      sym_parameter_name,
  [842] = 4,
    ACTIONS(111), 1,
      sym_identifier,
    ACTIONS(115), 1,
      anon_sym_RBRACE,
    STATE(66), 1,
      sym_struct_field,
    STATE(105), 1,
      sym_field_name,
  [855] = 4,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(117), 1,
      anon_sym_RPAREN,
    STATE(73), 1,
      sym_parameter,
    STATE(95), 1,
      sym_parameter_name,
  [868] = 3,
    ACTIONS(119), 1,
      anon_sym_RBRACE,
    ACTIONS(121), 1,
      anon_sym_COMMA,
    STATE(50), 1,
      aux_sym_struct_field_list_repeat1,
  [878] = 3,
    ACTIONS(109), 1,
      anon_sym_RBRACE,
    ACTIONS(123), 1,
      anon_sym_COMMA,
    STATE(54), 1,
      aux_sym_struct_field_list_repeat1,
  [888] = 3,
    ACTIONS(83), 1,
      sym_identifier,
    STATE(73), 1,
      sym_parameter,
    STATE(95), 1,
      sym_parameter_name,
  [898] = 3,
    ACTIONS(81), 1,
      sym_identifier,
    STATE(33), 1,
      sym_enum_value_name,
    STATE(67), 1,
      sym_enum_value_definition,
  [908] = 1,
    ACTIONS(125), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LPAREN,
  [914] = 3,
    ACTIONS(127), 1,
      anon_sym_RBRACE,
    ACTIONS(129), 1,
      anon_sym_COMMA,
    STATE(54), 1,
      aux_sym_struct_field_list_repeat1,
  [924] = 3,
    ACTIONS(132), 1,
      anon_sym_COMMA,
    ACTIONS(134), 1,
      anon_sym_RPAREN,
    STATE(57), 1,
      aux_sym_other_parameter_list_repeat1,
  [934] = 3,
    ACTIONS(111), 1,
      sym_identifier,
    STATE(66), 1,
      sym_struct_field,
    STATE(105), 1,
      sym_field_name,
  [944] = 3,
    ACTIONS(113), 1,
      anon_sym_RPAREN,
    ACTIONS(136), 1,
      anon_sym_COMMA,
    STATE(62), 1,
      aux_sym_other_parameter_list_repeat1,
  [954] = 3,
    ACTIONS(138), 1,
      anon_sym_RBRACE,
    ACTIONS(140), 1,
      anon_sym_COMMA,
    STATE(60), 1,
      aux_sym_enum_value_definition_list_repeat1,
  [964] = 3,
    ACTIONS(142), 1,
      anon_sym_message,
    STATE(75), 1,
      sym_message_parameter,
    STATE(81), 1,
      sym_parameter_list,
  [974] = 3,
    ACTIONS(101), 1,
      anon_sym_RBRACE,
    ACTIONS(144), 1,
      anon_sym_COMMA,
    STATE(61), 1,
      aux_sym_enum_value_definition_list_repeat1,
  [984] = 3,
    ACTIONS(146), 1,
      anon_sym_RBRACE,
    ACTIONS(148), 1,
      anon_sym_COMMA,
    STATE(61), 1,
      aux_sym_enum_value_definition_list_repeat1,
  [994] = 3,
    ACTIONS(151), 1,
      anon_sym_COMMA,
    ACTIONS(154), 1,
      anon_sym_RPAREN,
    STATE(62), 1,
      aux_sym_other_parameter_list_repeat1,
  [1004] = 1,
    ACTIONS(156), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1009] = 1,
    ACTIONS(158), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1014] = 2,
    ACTIONS(160), 1,
      anon_sym_COMMA,
    ACTIONS(162), 1,
      anon_sym_RPAREN,
  [1021] = 1,
    ACTIONS(127), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1026] = 1,
    ACTIONS(146), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1031] = 1,
    ACTIONS(164), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1036] = 2,
    ACTIONS(166), 1,
      anon_sym_to,
    STATE(65), 1,
      sym_to_parameter,
  [1043] = 2,
    ACTIONS(168), 1,
      sym_identifier,
    STATE(86), 1,
      sym_struct_name,
  [1050] = 2,
    ACTIONS(170), 1,
      sym_identifier,
    STATE(83), 1,
      sym_enum_name,
  [1057] = 2,
    ACTIONS(172), 1,
      sym_identifier,
    STATE(109), 1,
      sym_action_handle_name,
  [1064] = 1,
    ACTIONS(154), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1069] = 1,
    ACTIONS(174), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1074] = 2,
    ACTIONS(176), 1,
      anon_sym_COMMA,
    ACTIONS(178), 1,
      anon_sym_RPAREN,
  [1081] = 1,
    ACTIONS(180), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1086] = 1,
    ACTIONS(182), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1091] = 2,
    ACTIONS(184), 1,
      sym_identifier,
    STATE(96), 1,
      sym_automation_name,
  [1098] = 2,
    ACTIONS(186), 1,
      sym_identifier,
    STATE(15), 1,
      sym_type_name,
  [1105] = 1,
    ACTIONS(188), 1,
      anon_sym_LT,
  [1109] = 1,
    ACTIONS(190), 1,
      anon_sym_RPAREN,
  [1113] = 1,
    ACTIONS(192), 1,
      anon_sym_LBRACE,
  [1117] = 1,
    ACTIONS(194), 1,
      anon_sym_LBRACE,
  [1121] = 1,
    ACTIONS(196), 1,
      anon_sym_LBRACE,
  [1125] = 1,
    ACTIONS(198), 1,
      ts_builtin_sym_end,
  [1129] = 1,
    ACTIONS(200), 1,
      anon_sym_LBRACE,
  [1133] = 1,
    ACTIONS(202), 1,
      anon_sym_COLON,
  [1137] = 1,
    ACTIONS(204), 1,
      anon_sym_GT,
  [1141] = 1,
    ACTIONS(206), 1,
      anon_sym_GT,
  [1145] = 1,
    ACTIONS(208), 1,
      anon_sym_COLON,
  [1149] = 1,
    ACTIONS(210), 1,
      anon_sym_COLON,
  [1153] = 1,
    ACTIONS(212), 1,
      anon_sym_GT,
  [1157] = 1,
    ACTIONS(214), 1,
      anon_sym_RPAREN,
  [1161] = 1,
    ACTIONS(216), 1,
      sym_identifier,
  [1165] = 1,
    ACTIONS(218), 1,
      anon_sym_COLON,
  [1169] = 1,
    ACTIONS(220), 1,
      anon_sym_LBRACE,
  [1173] = 1,
    ACTIONS(222), 1,
      anon_sym_NodeId,
  [1177] = 1,
    ACTIONS(224), 1,
      anon_sym_COLON,
  [1181] = 1,
    ACTIONS(226), 1,
      anon_sym_RPAREN,
  [1185] = 1,
    ACTIONS(228), 1,
      anon_sym_LT,
  [1189] = 1,
    ACTIONS(230), 1,
      anon_sym_COLON,
  [1193] = 1,
    ACTIONS(232), 1,
      anon_sym_RBRACE,
  [1197] = 1,
    ACTIONS(234), 1,
      anon_sym_to,
  [1201] = 1,
    ACTIONS(236), 1,
      anon_sym_NodeId,
  [1205] = 1,
    ACTIONS(238), 1,
      anon_sym_COLON,
  [1209] = 1,
    ACTIONS(240), 1,
      anon_sym_LT,
  [1213] = 1,
    ACTIONS(242), 1,
      anon_sym_LBRACE,
  [1217] = 1,
    ACTIONS(244), 1,
      anon_sym_RBRACE,
  [1221] = 1,
    ACTIONS(246), 1,
      anon_sym_LPAREN,
  [1225] = 1,
    ACTIONS(248), 1,
      anon_sym_LPAREN,
  [1229] = 1,
    ACTIONS(250), 1,
      anon_sym_RPAREN,
  [1233] = 1,
    ACTIONS(252), 1,
      anon_sym_RBRACE,
};

static const uint64_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 57,
  [SMALL_STATE(4)] = 114,
  [SMALL_STATE(5)] = 171,
  [SMALL_STATE(6)] = 228,
  [SMALL_STATE(7)] = 285,
  [SMALL_STATE(8)] = 342,
  [SMALL_STATE(9)] = 399,
  [SMALL_STATE(10)] = 425,
  [SMALL_STATE(11)] = 451,
  [SMALL_STATE(12)] = 461,
  [SMALL_STATE(13)] = 471,
  [SMALL_STATE(14)] = 481,
  [SMALL_STATE(15)] = 491,
  [SMALL_STATE(16)] = 501,
  [SMALL_STATE(17)] = 511,
  [SMALL_STATE(18)] = 521,
  [SMALL_STATE(19)] = 531,
  [SMALL_STATE(20)] = 547,
  [SMALL_STATE(21)] = 557,
  [SMALL_STATE(22)] = 567,
  [SMALL_STATE(23)] = 577,
  [SMALL_STATE(24)] = 587,
  [SMALL_STATE(25)] = 597,
  [SMALL_STATE(26)] = 607,
  [SMALL_STATE(27)] = 623,
  [SMALL_STATE(28)] = 639,
  [SMALL_STATE(29)] = 658,
  [SMALL_STATE(30)] = 671,
  [SMALL_STATE(31)] = 684,
  [SMALL_STATE(32)] = 697,
  [SMALL_STATE(33)] = 704,
  [SMALL_STATE(34)] = 715,
  [SMALL_STATE(35)] = 728,
  [SMALL_STATE(36)] = 735,
  [SMALL_STATE(37)] = 742,
  [SMALL_STATE(38)] = 749,
  [SMALL_STATE(39)] = 756,
  [SMALL_STATE(40)] = 763,
  [SMALL_STATE(41)] = 776,
  [SMALL_STATE(42)] = 789,
  [SMALL_STATE(43)] = 796,
  [SMALL_STATE(44)] = 803,
  [SMALL_STATE(45)] = 816,
  [SMALL_STATE(46)] = 829,
  [SMALL_STATE(47)] = 842,
  [SMALL_STATE(48)] = 855,
  [SMALL_STATE(49)] = 868,
  [SMALL_STATE(50)] = 878,
  [SMALL_STATE(51)] = 888,
  [SMALL_STATE(52)] = 898,
  [SMALL_STATE(53)] = 908,
  [SMALL_STATE(54)] = 914,
  [SMALL_STATE(55)] = 924,
  [SMALL_STATE(56)] = 934,
  [SMALL_STATE(57)] = 944,
  [SMALL_STATE(58)] = 954,
  [SMALL_STATE(59)] = 964,
  [SMALL_STATE(60)] = 974,
  [SMALL_STATE(61)] = 984,
  [SMALL_STATE(62)] = 994,
  [SMALL_STATE(63)] = 1004,
  [SMALL_STATE(64)] = 1009,
  [SMALL_STATE(65)] = 1014,
  [SMALL_STATE(66)] = 1021,
  [SMALL_STATE(67)] = 1026,
  [SMALL_STATE(68)] = 1031,
  [SMALL_STATE(69)] = 1036,
  [SMALL_STATE(70)] = 1043,
  [SMALL_STATE(71)] = 1050,
  [SMALL_STATE(72)] = 1057,
  [SMALL_STATE(73)] = 1064,
  [SMALL_STATE(74)] = 1069,
  [SMALL_STATE(75)] = 1074,
  [SMALL_STATE(76)] = 1081,
  [SMALL_STATE(77)] = 1086,
  [SMALL_STATE(78)] = 1091,
  [SMALL_STATE(79)] = 1098,
  [SMALL_STATE(80)] = 1105,
  [SMALL_STATE(81)] = 1109,
  [SMALL_STATE(82)] = 1113,
  [SMALL_STATE(83)] = 1117,
  [SMALL_STATE(84)] = 1121,
  [SMALL_STATE(85)] = 1125,
  [SMALL_STATE(86)] = 1129,
  [SMALL_STATE(87)] = 1133,
  [SMALL_STATE(88)] = 1137,
  [SMALL_STATE(89)] = 1141,
  [SMALL_STATE(90)] = 1145,
  [SMALL_STATE(91)] = 1149,
  [SMALL_STATE(92)] = 1153,
  [SMALL_STATE(93)] = 1157,
  [SMALL_STATE(94)] = 1161,
  [SMALL_STATE(95)] = 1165,
  [SMALL_STATE(96)] = 1169,
  [SMALL_STATE(97)] = 1173,
  [SMALL_STATE(98)] = 1177,
  [SMALL_STATE(99)] = 1181,
  [SMALL_STATE(100)] = 1185,
  [SMALL_STATE(101)] = 1189,
  [SMALL_STATE(102)] = 1193,
  [SMALL_STATE(103)] = 1197,
  [SMALL_STATE(104)] = 1201,
  [SMALL_STATE(105)] = 1205,
  [SMALL_STATE(106)] = 1209,
  [SMALL_STATE(107)] = 1213,
  [SMALL_STATE(108)] = 1217,
  [SMALL_STATE(109)] = 1221,
  [SMALL_STATE(110)] = 1225,
  [SMALL_STATE(111)] = 1229,
  [SMALL_STATE(112)] = 1233,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(79),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(100),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(80),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(70),
  [30] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(71),
  [33] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(78),
  [36] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_set_type, 4, .production_id = 22),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_internal_type, 1, .production_id = 16),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_message_type, 4, .production_id = 22),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sequence_type, 4, .production_id = 22),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_user_define_type, 2, .production_id = 18),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_internal_type, 1, .production_id = 15),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_internal_type, 1, .production_id = 14),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data_type, 1, .production_id = 13),
  [52] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data_type, 1, .production_id = 12),
  [56] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data_type, 1, .production_id = 10),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_name, 1),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyword_primitive_type, 1),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_internal_type, 1, .production_id = 9),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_user_define_type, 1, .production_id = 17),
  [66] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_action_definition_list_repeat1, 2), SHIFT_REPEAT(94),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_action_definition_list_repeat1, 2),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_definition_list, 1),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(90),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(91),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_definition, 5, .production_id = 21),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_definition, 6, .production_id = 24),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value_definition, 1, .production_id = 4),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_automaton_or_type_definition, 1, .production_id = 1),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_automaton_or_type_definition, 1, .production_id = 2),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_definition, 5, .production_id = 5),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_handle_return, 2, .production_id = 19),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_definition, 5, .production_id = 6),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value_definition_list, 2),
  [103] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value_definition_list, 3),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_automaton_or_type_definition, 1, .production_id = 3),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_automaton_definition, 5, .production_id = 8),
  [109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_field_list, 2),
  [111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_other_parameter_list, 2),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_field_list, 3),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_other_parameter_list, 3),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_field_list, 1),
  [121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [123] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [125] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value_name, 1),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_struct_field_list_repeat1, 2),
  [129] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_struct_field_list_repeat1, 2), SHIFT_REPEAT(56),
  [132] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_other_parameter_list, 1),
  [136] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value_definition_list, 1),
  [140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [142] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [144] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [146] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_enum_value_definition_list_repeat1, 2),
  [148] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_enum_value_definition_list_repeat1, 2), SHIFT_REPEAT(52),
  [151] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_other_parameter_list_repeat1, 2), SHIFT_REPEAT(51),
  [154] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_other_parameter_list_repeat1, 2),
  [156] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter, 3, .production_id = 28),
  [158] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value_definition, 2, .production_id = 7),
  [160] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [162] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 4, .production_id = 26),
  [164] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_field, 3, .production_id = 11),
  [166] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [168] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [170] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [172] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [174] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_payload, 3, .production_id = 19),
  [176] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [178] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 1, .production_id = 20),
  [180] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_to_parameter, 3, .production_id = 27),
  [182] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_message_parameter, 3, .production_id = 23),
  [184] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [186] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [188] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [192] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_automation_name, 1),
  [194] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [196] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_name, 1),
  [198] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [200] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [210] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_name, 1),
  [212] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [214] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 3, .production_id = 25),
  [216] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_type, 1),
  [218] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [220] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [222] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [224] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [226] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [228] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [230] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_name, 1),
  [232] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [234] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_from_parameter, 3, .production_id = 27),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [238] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [240] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [242] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_name, 1),
  [244] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [248] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_handle_name, 1),
  [250] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 6, .production_id = 29),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_action(void) {
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
