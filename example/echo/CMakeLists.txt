cmake_minimum_required(VERSION 3.0)

project(
  echo
  VERSION 1.0
  LANGUAGES CXX)



include_directories(
    third_party/asio-1.28.0/include
)

add_executable(
    echo_client
    echo_client.cpp
)

add_executable(
    echo_server
    echo_server.cpp
)



