#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// Set up an automata
/// `_name`, the automata's name
/// `_tested_nid`, tested node's node id
/// `_player_nid`, deterministic player's node id
/// `_player_addr`, deterministic player's network address [ip:port]
void automata_setup(const char *_name,
                    uint64_t _tested_nid,
                    uint64_t _player_nid,
                    const char *_player_addr);

/// Set up an automata
/// `_name`, the automata's name
/// `_tested_nid`, tested node's node id
/// `_tested_nid`, tested node's network address [ip:port]
/// `_player_nid`, deterministic player's node id
/// `_player_addr`, deterministic player's network address [ip:port]
void automata_setup_with_input(const char *_name,
                               uint64_t _tested_nid,
                               const char *_tested_addr,
                               uint64_t _player_nid,
                               const char *_player_addr);

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
void automata_action(const char *_name,
                     uint64_t _action_type,
                     uint64_t _action_begin_end,
                     uint64_t _source_node_id,
                     uint64_t _dest_node_id,
                     const char *_message_json_string);

} // extern "C"
