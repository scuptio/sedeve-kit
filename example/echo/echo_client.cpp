//
// echo_client.cpp
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// Copyright (c) 2003-2023 Christopher M. Kohlhoff (chris at kohlhoff dot com)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//
#define ENABLE_DTM

#include <cstring>
#include <iostream>
#include <boost/json.hpp>
#include <boost/asio.hpp>
#include "inst_context.h"
#include "sedeve_kit.h"



using boost::asio::io_context;
using boost::asio::connect;
using boost::asio::ip::tcp;
using boost::json::value;
using boost::json::serialize;
using boost::json::value_to;
using boost::asio::write;
using boost::asio::read;
using boost::asio::buffer;
using std::string;

enum {
    max_length = 1024
};

int main(int argc, char *argv[]) {
    try {
        if (argc != 4) {
            std::cerr << "Usage: blocking_tcp_echo_client <host> <port> <node_id>\n";
            return 1;
        }

        io_context io_context;

        tcp::socket s(io_context);
        tcp::resolver resolver(io_context);
        

        auto node_id = uint64_t(argv[3]);
        /*
        {
            value action_connect = {"type", "connect"};
            std::string json_connect =  boost::json::serialize(action_connect);
            OUTPUT_ACTION(AUTO_ECHO, node_id, node_id, json_connect.c_str());
        }*/
        connect(s, resolver.resolve(argv[1], argv[2]));

        std::cout << "Enter message: ";
        char request[max_length];
        std::cin.getline(request, max_length);
        size_t request_length = std::strlen(request);

#ifdef ENABLE_DTM
        {
            string message(request, request_length);
            value action_send_to_server = {
                        {"type", "send_to_server"},
                        {"message", message}
            };
            auto json_send_to_server = value_to<std::string>(action_send_to_server);
            OUTPUT_ACTION(AUTO_ECHO, node_id, node_id, json_send_to_server.c_str());
        }
#else
        write(s, buffer(request, request_length));
#endif

        char reply[max_length];
        size_t reply_length = 0;
#ifdef ENABLE_DTM
        {
            uint64_t source = 0;
            uint64_t dest = 0;
            uint64_t length = 0;
            uint64_t action_type = 0;
            auto ret = automata_next_action(AUTO_ECHO, &source, &dest, &action_type, reply, max_length, &length);
            if (ret != 0) {
                return -1;
            }
            reply_length = length;
            string message(request, reply_length);
            value action_send_to_server = {
                    {"type", "send_to_server"},
                    {"message", message}
            };
            auto json_send_to_server = value_to<std::string>(action_send_to_server);
            OUTPUT_ACTION(AUTO_ECHO, node_id, node_id, json_send_to_server.c_str());
        }
#else
        reply_length = read(s,
            buffer(reply, max_length));
#endif
        std::cout << "Reply is: ";
        std::cout.write(reply, reply_length);
        std::cout << "\n";
    }
    catch (std::exception &e) {
        std::string what = e.what();
        auto node_id = uint64_t(argv[3]);
        if (what == "connect: Connection refused") {
            value connection_refused = {
                        {"type", "connect_to_server_result"},
                        {"ok", false}
            };
            string json_connection_refused = serialize(connection_refused);
            INPUT_ACTION(AUTO_ECHO, node_id, node_id, json_connection_refused.c_str());
        }
        std::cerr << "Exception: " << what << "\n";
    }

    return 0;
}
