#define _GNU_SOURCE

// CPP stuff

#include "../inc/arg_parser.hpp"
#include "../inc/vm.hpp"
#include "../inc/constants.hpp"

#include <string>
#include <iostream>

#include <pthread.h>

void* vm_body_wrapper(void* arg) {

	__vm *vm = (__vm*)arg;

	__vm::vm_body(vm->vm_image, vm->mem_size, vm->page_size);

	return NULL;
}


int main(int argc, char *argv[])
{

	uint64_t mem_size, page_size;

	std::vector<std::string> guest_images;
	std::vector<std::string> shared_files;


	parse_args(argc, argv, guest_images, shared_files, &mem_size, &page_size);

	File_Controller::open_shared_files(shared_files);

	pthread_t threads[guest_images.size()];
	__vm          vms[guest_images.size()];
	for(uint64_t i = 0; i < guest_images.size(); ++i) {

		vms[i].mem_size = mem_size;
		vms[i].page_size = page_size;
		vms[i].vm_image = guest_images[i];

		int res = pthread_create(&threads[i], NULL, vm_body_wrapper, (void *)&vms[i]);
		if(res) {
			printf("Riko Kaboom!");
			return -1;
		}
	}

	for(uint64_t i = 0; i < guest_images.size(); ++i) {
		 pthread_join(threads[i], NULL);
	}

	File_Controller::close_shared_files(shared_files);

	return 0;
}
