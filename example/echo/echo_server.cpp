//
// echo_server.cpp
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// Copyright (c) 2003-2023 Christopher M. Kohlhoff (chris at kohlhoff dot com)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

#define ENABLE_DTM

#include <cstdlib>
#include <iostream>
#include <thread>
#include <utility>
#include <boost/json.hpp>
#include <boost/asio.hpp>
#include "sedeve_kit.h"

const char *AUTO_ECHO = "AUTO_ECHO";

using boost::asio::ip::tcp;
using boost::asio::buffer;
using boost::asio::stream_errc::eof;
using boost::system::error_code;
using boost::asio::io_context;

const int max_length = 1024;


void session(tcp::socket sock) {
    try {
        for (;;) {
            char data[max_length];

            error_code error;
            size_t length = sock.read_some(buffer(data), error);
            if (error == eof)
                break; // Connection closed cleanly by peer.
            else if (error)
                throw std::system_error(error); // Some other error.

            write(sock, buffer(data, length));
        }
    }
    catch (std::exception &e) {
        std::cerr << "Exception in thread: " << e.what() << "\n";
    }
}

[[noreturn]] void server(io_context &io_context, unsigned short port, uint64_t node_id) {
    INPUT_ACTION(AUTO_ECHO, node_id, node_id, "SERVER_START");
    tcp::acceptor a(io_context, tcp::endpoint(tcp::v4(), port));
    for (;;) {
        tcp::socket sock(io_context);
        a.accept(sock);
        std::thread(session, std::move(sock)).detach();
    }
}

int main(int argc, char *argv[]) {
    try {
        if (argc != 3) {
            std::cerr << "Usage: blocking_tcp_echo_server <port> <id>\n";
            return 1;
        }

        io_context io_context;

        server(io_context, std::atoi(argv[1]), uint64_t(std::atoi(argv[2])));
    }
    catch (std::exception &e) {
        std::cerr << "Exception: " << e.what() << "\n";
    }

    return 0;
}
