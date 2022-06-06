cmake_minimum_required(VERSION 3.23)

project(bf_emulator)

file(DOWNLOAD https://raw.githubusercontent.com/p-ranav/argparse/master/include/argparse/argparse.hpp ../src/argparse.hpp)

add_executable(cpp_bf_emulator src/main.cpp)
