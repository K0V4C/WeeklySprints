#ifndef VIRTUAL_MACHINE_STRUCT
#define VIRTUAL_MACHINE_STRUCT

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <string.h>
#include <stdint.h>
#include <linux/kvm.h>

#include "files.hpp"

class __vm {
public:
    int kvm_fd;
	int vm_fd;
	int vcpu_fd;
	char *mem;
	struct kvm_run *kvm_run;

    uint64_t mem_size;
    uint64_t page_size;

    std::string vm_image;

    // =====For working with files========

    File_Controller file_controller;

    uint64_t id;
    static uint64_t next_id;

    //====================================

    __vm();
    ~__vm();

    static int init_vm(__vm &vm, size_t mem_size, uint64_t page_size);

    static void setup_64bit_code_segment(struct kvm_sregs *sregs);

    static void setup_long_mode(__vm &vm, kvm_sregs *sregs);

    static int vm_body(std::string guest_image, uint64_t mem_size, uint64_t page_size);
};


#endif