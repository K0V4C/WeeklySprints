#include "../inc/arg_parser.hpp"

#include <string.h>
#include <iostream>

int parse_args(int argc, char** argv, std::vector<std::string> &images, uint64_t *_mem, uint64_t *_page) {
	
    uint64_t number_of_guests = 0;
    uint64_t start = 0;

    uint64_t memory = 2;
    uint64_t page = 2;

	if(argc == 1) {
        printf("Please provide args for memory, paging and guests");
		return -1;
    }

    int i = 1;
    while(i < argc) {

        if (strcmp(argv[i], "-p") == 0 || strcmp(argv[i], "--page") == 0 ) {

            if (i + 1 > argc) {
                printf("Number for page not provided");
                return -1;
            }

            page = atoi(argv[i+1]);
            i += 1;
        }

        
        if (strcmp(argv[i], "-m") == 0 || strcmp(argv[i], "--memory") == 0) {

            if (i + 1 > argc) {
                printf("Number for memory not provided");
                return -1;
            }

            memory = atoi(argv[i+1]);
            i += 1;
        }

        if(strcmp(argv[i], "-g") == 0 || strcmp(argv[i], "--guest") == 0) {

            i += 1;

            if(start != 0) {
                printf("Ne koriste se opcije na pravilan nacin!");
                return -1;
            }

            start = i;

            while(i < argc && argv[i][0] != '-') {
                number_of_guests += 1;
                i += 1;
            }
        
        }

        i += 1;
    }

    if(memory != 2 && memory != 4 && memory != 8) {
        printf("Ne podrzava se ova velicina memorije");
        return -1;
    }

    if(page != 2 && page != 4) {
        printf("Ne podrzava se ova velicina stranice");
        return -1;
    }

    if(number_of_guests == 0) {
        printf("Nema gostiju!");
        return -1;
    }

    for(int i = 0; i <number_of_guests; i++) {
        images.push_back(argv[start + i]);
    }

    *_mem = memory * 1024 * 1024;

    if(page == 4) {
        *_page = page * 1024;
    } else if(page == 2) {
        *_page = page * 1024 * 1024;
    }

	return 0;
}