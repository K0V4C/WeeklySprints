#ifndef ARGUMENT_PARSER_HPP
#define ARGUMENT_PARSER_HPP

#include <string>
#include <vector>

int parse_args(int argc, char** argv, std::vector<std::string>&, uint64_t* _mem, uint64_t* _page);

#endif