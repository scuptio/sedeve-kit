#include "inst_context.h"


shared_ptr<inst_context> _context = nullptr;


void create_context(uint64_t node_id) {
    std::shared_ptr<inst_context> ctx(new inst_context(node_id));
    _context = ctx;
}

shared_ptr<sync_queue<message>> create_sync_queue_for_remote(uint64_t node_id) {
    return _context->create_sync_queue_for_remote(node_id);
}

shared_ptr<sync_queue<message>> get_sync_queue_for_remote(uint64_t node_id) {
    return _context->get_sync_queue_for_remote(node_id);
}

shared_ptr<sync_queue<message>> global_channel() {
    return _context->global_channel();
}

inst_context::inst_context(uint64_t node_id): node_id_(node_id) {

}

shared_ptr<sync_queue<message>> inst_context::global_channel() {
    return global_channel_;
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