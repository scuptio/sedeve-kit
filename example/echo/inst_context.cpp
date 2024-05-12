#include "inst_context.h"


shared_ptr<inst_context> context = nullptr;


void create_context() {
    std::shared_ptr<inst_context> ctx(new inst_context());
    context = ctx;
}

shared_ptr<sync_queue<message>> create_sync_queue_for_remote(uint64_t node_id) {
    return context->create_sync_queue_for_remote(node_id);
}

shared_ptr<sync_queue<message>> get_sync_queue_for_remote(uint64_t node_id) {
    return context->get_sync_queue_for_remote(node_id);
}

shared_ptr<sync_queue<message>> global_channel() {
    return context->global_channel();
}

inst_context::inst_context() {
    std::shared_ptr<sync_queue<message>> queue(new sync_queue<message>);
    global_channel_ = queue;
}

shared_ptr<sync_queue<message>> inst_context::global_channel() {
    return global_channel_;
}


shared_ptr<sync_queue<message>> inst_context::create_sync_queue_for_remote(uint64_t node_id) {
    std::unique_lock lock(mutex_);
    std::shared_ptr<sync_queue<message>> queue(new sync_queue<message>);
    channel_.insert(std::make_pair(node_id, queue));
    return queue;
}

shared_ptr<sync_queue<message>> inst_context::get_sync_queue_for_remote(uint64_t node_id) {
    std::unique_lock lock(mutex_);
    auto iter = channel_.find(node_id);
    if (iter != channel_.end()) {
        return iter->second;
    } else {
        return nullptr;
    }
}