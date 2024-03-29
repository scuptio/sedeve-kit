cmake_minimum_required(VERSION 3.0)

project(
    echo
    VERSION 1.0
    LANGUAGES CXX
)

include_directories(
    third_party/asio-1.28.0/include
    ../../out/include
)


link_directories(
    ../../out/lib
    ../../target/debug
    ../../target/release
)

add_executable(
    echo_client
    inst_context.cpp
    echo_client.cpp
)

add_executable(
    echo_server
    inst_context.cpp
    echo_server.cpp
)

target_link_libraries(
    echo_server
    sedeve_kit
    boost_json
    boost_thread
)

target_link_libraries(
    echo_client
    sedeve_kit
    boost_json
    boost_thread
)
