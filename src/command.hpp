#pragma once
#include <iostream>
#include <exception>

namespace cmd {

enum Command {
    Next = 0,
    Previous,
    Increment,
    Decrement,
    Put,
    Read,
    LoopBegin,
    LoopEnd,
};

enum Command parse(char token) {
    switch (token) {
        case '>': return Command::Next;
        case '<': return Command::Previous;
        case '+': return Command::Increment;
        case '-': return Command::Decrement;
        case '.': return Command::Put;
        case ',': return Command::Read;
        case '[': return Command::LoopBegin;
        case ']': return Command::LoopEnd;
    }
    throw std::runtime_error(std::string("unknown token `") + token + std::string("`"));
}

bool is_valid_token(const char t) {
    return (t == '>') || (t == '<') || (t == '+') || (t == '-') || (t == '.') || (t == ',') || (t == '[') || (t == ']');
}

}
