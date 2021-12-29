#pragma once
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <stack>
#include <map>

class Emulator {
    std::map<std::size_t, std::size_t> jumps;
    std::stack<std::size_t> stack;
    std::vector<cmd::Command> app;
    std::vector<char> mem;
    std::size_t ip = 0;
    std::size_t ir = 0;
public:
    Emulator(std::size_t mem_size) : mem(mem_size, 0x00) {
        app.reserve(consts::DEFAULT_APP_BUFFER);
    }

    void parse_token(char token) {
        if (cmd::is_valid_token(token)) {
            std::size_t index = app.size();
            cmd::Command cmd = cmd::parse(token);
            app.push_back(cmd);

            if (cmd == cmd::Command::LoopBegin) {
                stack.push(index);
            }
            if (cmd == cmd::Command::LoopEnd) {
                std::size_t start = stack.top();
                stack.pop();
                jumps[start] = index;
                jumps[index] = start;
            }
            index += 1;
        }
    }

    void from_buffer(const std::string &buffer) {
        for (auto token : buffer) {
            parse_token(token);
        }

        if (stack.size() != 0) {
            throw std::runtime_error("number of brackets does not match");
        }
    }

    void from_file(const char *filename) {
        std::ifstream fp(filename);

        if (!fp.is_open()) {
            throw std::runtime_error(std::string("fail to open ") + filename);
        }

        while (not fp.eof()) {
            parse_token(fp.get());
        }

        if (stack.size() != 0) {
            throw std::runtime_error("number of brackets does not match");
        }
    }

    void step() {
        switch (app[ip]) {
            case cmd::Command::Next: {
                if (ir + 1 > mem.size()) {
                    throw std::runtime_error("out of memory");
                }
                ir += 1;
                break;
            }
            case cmd::Command::Previous: {
                if (ir == 0) {
                    throw std::runtime_error("cannot access negative memory index");
                }
                ir -= 1;
                break;
            }
            case cmd::Command::Increment: {
                mem[ir] += 1;
                break;
            }
            case cmd::Command::Decrement: {
                mem[ir] -= 1;
                break;
            }
            case cmd::Command::Put: {
                std::cout << mem[ir];
                std::cout.flush();
                break;
            }
            case cmd::Command::Read: {
                mem[ir] = std::cin.get();
                break;
            }
            case cmd::Command::LoopBegin: {
                if (mem[ir] == 0) {
                    ip = jumps[ip];
                }
                break;
            }
            case cmd::Command::LoopEnd: {
                if (mem[ir] != 0) {
                    ip = jumps[ip];
                }
                break;
            }
        }
        ip += 1;
    }

    void execute() {
        while (ip < app.size()) {
            step();
        }
    }
};
