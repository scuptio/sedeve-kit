#include "inst_context.h"



uint64_t endpoint_to_id(const string &addr, uint16_t port) {
    return 0;
}

shared_ptr<sync_queue<message>> create_sync_queue_for_remote(uint64_t node_id) {
    return nullptr;
}

shared_ptr<sync_queue<message>> get_sync_queue_for_remote(uint64_t node_id) {
    return nullptr;
}

inst_context::inst_context(uint64_t node_id): node_id_(node_id) {

}

uint64_t inst_context::endpoint_to_id(const string &addr, uint16_t port) {
    string s = addr + ":" + std::to_string(port);
    auto iter = node_id_.find(s);
    if (iter != node_id_.end()) {
        return iter->second;
    } else {
        return 0;
    }
}


void inst_context::add_node(const string&addr, uint16_t port, uint64_t node_id) {
    string s = addr + ":" + std::to_string(port);
    node_id_.insert(std::make_pair(s, node_id));
}

shared_ptr<sync_queue<message>> inst_context::create_sync_queue_for_remote(uint64_t node_id) {
    std::shared_ptr<sync_queue<message>> queue(new sync_queue<message>);
    channel_.insert(std::make_pair(node_id, queue));
    return queue;
}

shared_ptr<sync_queue<message>> inst_context::get_sync_queue_for_remote(uint64_t node_id) {
    auto iter = channel_.find(node_id);
    if (iter != channel_.end()) {
        return iter->second;
    } else {
        return nullptr;
    }
}