#pragma once


#include <boost/asio.hpp>
#include <boost/thread/concurrent_queues/sync_queue.hpp>
#include <memory>

using boost::asio::ip::basic_endpoint;
using std::string;
using boost::concurrent::sync_queue;
using std::pair;
using std::shared_ptr;


static const char *AUTO_ECHO = "AUTO_ECHO";

struct message {
    uint64_t source;
    uint64_t dest;
    string json;
};



class inst_context {
private:
    std::mutex mutex_;
    std::unordered_map<uint64_t, shared_ptr<sync_queue<message>>> channel_;

    shared_ptr<sync_queue<message>> global_channel_;
public:
    inst_context();

    shared_ptr<sync_queue<message>> global_channel();
    shared_ptr<sync_queue<message>> create_sync_queue_for_remote(uint64_t node_id);
    shared_ptr<sync_queue<message>> get_sync_queue_for_remote(uint64_t node_id);
};






void create_context();
shared_ptr<sync_queue<message>> global_channel();
shared_ptr<sync_queue<message>> create_sync_queue_for_remote(uint64_t node_id);
shared_ptr<sync_queue<message>> get_sync_queue_for_remote(uint64_t node_id);
