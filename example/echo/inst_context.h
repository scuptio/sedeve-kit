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
    string type;
    string json;
};



class inst_context {
private:
    std::mutex mutex_;
    std::unordered_map<uint64_t, shared_ptr<sync_queue<message>>> channel_;
    std::unordered_map<string, uint64_t> node_id_;
    shared_ptr<sync_queue<message>> global_channel_;
public:
    explicit inst_context(uint64_t node_id);

    void add_node(const string&addr, uint16_t port, uint64_t node_id);
    shared_ptr<sync_queue<message>> global_channel();
    shared_ptr<sync_queue<message>> create_sync_queue_for_remote(uint64_t node_id);
    shared_ptr<sync_queue<message>> get_sync_queue_for_remote(uint64_t node_id);
};




uint64_t endpoint_to_id(const string &addr, uint16_t port);

void create_context(uint64_t node_id);
shared_ptr<sync_queue<message>> global_channel();
shared_ptr<sync_queue<message>> create_sync_queue_for_remote(uint64_t node_id);
shared_ptr<sync_queue<message>> get_sync_queue_for_remote(uint64_t node_id);
