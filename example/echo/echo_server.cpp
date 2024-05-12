//
// echo_server.cpp
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// Copyright (c) 2003-2023 Christopher M. Kohlhoff (chris at kohlhoff dot com)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

#define ENABLE_DTM // to enable deterministic testing

#include <cstdlib>
#include <iostream>
#include <thread>
#include <utility>
#include <boost/json.hpp>
#include <boost/asio.hpp>
#include <boost/thread/concurrent_queues/sync_queue.hpp>
#include "sedeve_kit.h"
#include "inst_context.h"

using boost::asio::ip::tcp;
using boost::asio::buffer;
using boost::asio::stream_errc::eof;
using boost::system::error_code;
using boost::asio::io_context;
using boost::json::value;
using boost::json::object;
using boost::json::value_to;
using boost::json::parse;
using boost::concurrent::sync_queue;

const int max_length = 1024;

std::pair<std::string, std::string> message_type(std::string &j) {
    boost::system::error_code ec;
    value jv = parse(j, ec);
    if (ec)
        std::cout << "Parsing failed: " << ec.message() << "\n";

    if (jv.if_object()) {
        object o = jv.as_object();
        if (o.size() != 1) {
            std::cerr << "json format error";
        }

        for (auto &i: o) {
            std::string key = i.key();
            std::string value = i.value().as_string().c_str();
            return std::make_pair(key, value);
        }
        return std::make_pair("", "");
    } else if (jv.is_string()) {
        std::string type = jv.as_string().c_str();
        return std::make_pair(type, "");
    } else {
        return std::make_pair("", "");
    }

}

void session(
        tcp::socket sock,
        uint64_t local,
        uint64_t remote,
        shared_ptr<sync_queue<message>> queue
) {
    try {
        for (;;) {
            char data[max_length];
            error_code error;
            size_t length;

#ifdef ENABLE_DTM
            auto message = queue->pull();
            std::cout << message.json;
#endif
#ifdef ENABLE_DTM
            std::string message_payload;
            {

                if (message.json.length() > max_length) {
                    break;
                }
                auto p = message_type(message.json);
                if (p.first == "stop") {
                    automata_close_input(AUTO_ECHO);
                }
                INPUT_ACTION(AUTO_ECHO, remote, local, message.json.c_str());

                if (p.first == "stop") {
                    automata_clear(AUTO_ECHO);
                    exit(0);
                } else if (p.first == "request") {
                    message_payload = p.second;
                } else {
                    exit(-1);
                }
            }
#else
            length = sock.read_some(buffer(data), error);
            if (error == eof)
                break; // Connection closed cleanly by peer.
            else if (error)
                throw std::system_error(error); // Some other error.
#endif

#ifdef ENABLE_DTM
            {
                value response = {
                        {"response", message_payload}
                };

                auto json_reply_to_client = serialize(response);
                OUTPUT_ACTION(AUTO_ECHO, local, remote, json_reply_to_client.c_str());
            }
#else
            write(sock, buffer(data, length));
#endif
        }
    }
    catch (std::exception &e) {
        std::cerr << "Exception in thread: " << e.what() << "\n";
    }
}

int loop_incoming_input_from_player() {
    uint64_t _output_source_node_id = 0;
    uint64_t _output_dest_node_id = 0;
    char _output_buf_output_action_json[40960];
    uint64_t _buf_len = sizeof(_output_buf_output_action_json);
    uint64_t _output_len;
    while (true) {
        int ret = automata_next_input(
                AUTO_ECHO,
                &_output_source_node_id,
                &_output_dest_node_id,
                _output_buf_output_action_json,
                _buf_len,
                &_output_len);
        if (ret < 0) {
            std::cerr << "automata_next_input error, return value " << ret;
            return ret;
        }
        auto json = std::string(_output_buf_output_action_json, _output_len);
        auto queue = get_sync_queue_for_remote(_output_source_node_id);
        message m;
        m.source = _output_source_node_id;
        m.dest = _output_dest_node_id;
        m.json = json;
        if (queue == nullptr) {
            queue = create_sync_queue_for_remote(_output_source_node_id);
            auto ch = global_channel();
            ch->push(m);
        }
        queue->push(m);
    }
}

[[noreturn]] void server(io_context &io_context, unsigned short port, uint64_t local_id) {
    std::thread(loop_incoming_input_from_player).detach();

    tcp::acceptor a(io_context, tcp::endpoint(tcp::v4(), port));
    for (;;) {
        tcp::socket sock(io_context);
        uint64_t remote_id = 0;
#ifdef ENABLE_DTM
        auto message = global_channel()->pull();
        remote_id = message.source;
#else
        a.accept(sock);
        auto endpoint = sock.remote_endpoint();
#endif
        auto queue = get_sync_queue_for_remote(remote_id);
        assert(queue);
        std::thread(session, std::move(sock), local_id, remote_id, std::move(queue)).detach();
    }
}

int main(int argc, char *argv[]) {
    try {
        if (argc != 7) {
            std::cerr << "Usage: echo_server <port> <id> <dtm tested port> <player id> <player ip> <player port>\n";
            return 1;
        }

        uint16_t port = std::atoi(argv[1]);
        uint64_t node_id = uint64_t(std::atoi(argv[2]));
        uint16_t dtm_t_port = std::atoi(argv[3]);
        uint64_t player_id = uint64_t(std::atoi(argv[4]));
        std::string player_ip = argv[5];
        uint16_t player_port = std::atoi(argv[6]);

        std::string tested_addr = "0.0.0.0:" + std::to_string(dtm_t_port);
        std::string player_addr = player_ip + ":" + std::to_string(player_port);

#ifdef ENABLE_DTM
        create_context();
        automata_setup_with_input(AUTO_ECHO, node_id, tested_addr.c_str(), player_id, player_addr.c_str());
#endif

        io_context io_context;
        server(io_context, port, node_id);
    }
    catch (std::exception &e) {
        std::cerr << "Exception: " << e.what() << "\n";
    }

    return 0;
}


