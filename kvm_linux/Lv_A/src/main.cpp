#define _GNU_SOURCE

// CPP stuff

#include "../inc/arg_parser.hpp"
#include "../inc/vm.hpp"
#include "../inc/constants.hpp"

#include <string>
#include <iostream>


int main(int argc, char *argv[])
{

	uint64_t mem_size, page_size;

	std::vector<std::string> guest_images;

	parse_args(argc, argv, guest_images, &mem_size, &page_size);
	
	__vm::vm_body(guest_images[0], mem_size, page_size);

	return 0;
}
