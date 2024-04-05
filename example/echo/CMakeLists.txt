cmake_minimum_required(VERSION 3.0)

project(
    echo
    VERSION 1.0
    LANGUAGES CXX
)

include_directories(
    ../../binding/automata_c/out/include
)


link_directories(
    ../../binding/automata_c/out/lib
    ../../binding/automata_c/target/debug
    ../../binding/automata_c/target/release
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
    automata_c
    boost_json
    boost_thread
)

target_link_libraries(
    echo_client
    automata_c
    boost_json
    boost_thread
)
