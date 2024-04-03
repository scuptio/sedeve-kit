#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// Initialize an automata
/// `_name`, the automata's name
/// `_client_id`, client node id
/// `_server_id`, server(dtm player) node id
/// `_server_addr`, server(deterministic player to connect) network address [ip:port]
void automata_init_setup(const char *_name,
                         uint64_t _client_id,
                         uint64_t _server_id,
                         const char *_server_addr);

/// Clean an automata setting
/// `_name`, the automata's name
void automata_clear(const char *_name);

/// Is an automata named `name` enable
int32_t automata_enable(const char *_name);

/// Read next input action of automata, receive message from deterministic player
/// `_name`, the automata's name
/// `_output_source_node_id`, source node id
/// `_output_dest_node_id`, dest node id
/// `_output_action_type`, action type
/// `_output_buf_output_action_json`, output buffer
/// `_buf_len`, the buffer length
/// `_output_len`, write bytes to output buffer
int32_t automata_next_input(const char *_name,
                            uint64_t *_output_source_node_id,
                            uint64_t *_output_dest_node_id,
                            uint64_t *_output_action_type,
                            char *_output_buf_output_action_json,
                            uint64_t _buf_len,
                            uint64_t *_output_len);

/// Send an action to deterministic player
/// `_name`, the automata's name
/// `_action_type`, the action type, input/internal/output
/// `_action_begin_end`, begin an action, or end an action
/// `_source_node_id`, source node id
/// `_dest_node_id`, dest node id
/// `_message_json_string`, message in json reprensentaion
void automata_action_to_player(const char *_name,
                               uint64_t _action_type,
                               uint64_t _action_begin_end,
                               uint64_t _source_node_id,
                               uint64_t _dest_node_id,
                               const char *_message_json_string);

} // extern "C"
