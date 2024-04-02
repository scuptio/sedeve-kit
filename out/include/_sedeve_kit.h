#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint64_t C_ACTION_BEGIN = 0;

constexpr static const uint64_t C_ACTION_END = 1;

constexpr static const uint64_t C_ACTION_SETUP = 0;

constexpr static const uint64_t C_ACTION_CHECK = 1;

constexpr static const uint64_t C_ACTION_INPUT = 2;

constexpr static const uint64_t C_ACTION_OUTPUT = 3;

constexpr static const uint64_t C_ACTION_INTERNAL = 4;

extern "C" {

int32_t automata_next_action(const char *_name,
                             uint64_t *_output_source_node_id,
                             uint64_t *_output_dest_node_id,
                             uint64_t *_output_action_type,
                             char *_output_buf_output_action_json,
                             uint64_t _buf_len,
                             uint64_t *_output_len);

void automata_action(const char *_name,
                     uint64_t _action_type,
                     uint64_t _action_begin_end,
                     uint64_t _source_node_id,
                     uint64_t _dest_node_id,
                     const char *_message_json_string);

} // extern "C"
