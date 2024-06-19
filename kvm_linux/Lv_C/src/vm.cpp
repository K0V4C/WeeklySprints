#include "../inc/vm.hpp"
#include "../inc/constants.hpp"

#include <iostream>

uint64_t __vm::next_id = 0;

__vm::__vm() {}

int __vm::init_vm(__vm &vm, size_t mem_size, uint64_t page_size) {

    struct kvm_userspace_memory_region region;
	int kvm_run_mmap_size;

    vm.mem_size = mem_size;
    vm.page_size = page_size;

	vm.kvm_fd = open("/dev/kvm", O_RDWR);
	if (vm.kvm_fd < 0) {
		perror("open /dev/kvm");
		return -1;
	}

	vm.vm_fd = ioctl(vm.kvm_fd, KVM_CREATE_VM, 0);
	if (vm.vm_fd < 0) {
		perror("KVM_CREATE_VM");
		return -1;
	}

	vm.mem = (char*) mmap(NULL, mem_size, PROT_READ | PROT_WRITE,
		   MAP_SHARED | MAP_ANONYMOUS, -1, 0);
	if (vm.mem == MAP_FAILED) {
		perror("mmap mem");
		return -1;
	}

	region.slot = 0;
	region.flags = 0;
	region.guest_phys_addr = 0;
	region.memory_size = mem_size;
	region.userspace_addr = (unsigned long)vm.mem;
    if (ioctl(vm.vm_fd, KVM_SET_USER_MEMORY_REGION, &region) < 0) {
		perror("KVM_SET_USER_MEMORY_REGION");
        return -1;
	}

	vm.vcpu_fd = ioctl(vm.vm_fd, KVM_CREATE_VCPU, 0);
    if (vm.vcpu_fd < 0) {
		perror("KVM_CREATE_VCPU");
        return -1;
	}

	kvm_run_mmap_size = ioctl(vm.kvm_fd, KVM_GET_VCPU_MMAP_SIZE, 0);
    if (kvm_run_mmap_size <= 0) {
		perror("KVM_GET_VCPU_MMAP_SIZE");
		return -1;
	}

	vm.kvm_run = (struct kvm_run*) mmap(NULL, kvm_run_mmap_size, PROT_READ | PROT_WRITE,
			     MAP_SHARED, vm.vcpu_fd, 0);
	if (vm.kvm_run == MAP_FAILED) {
		perror("mmap kvm_run");
		return -1;
	}

	File_Controller::setup_shared_cursors(vm.id);

	return 0;
}

void __vm::setup_64bit_code_segment(kvm_sregs *sregs) {
	struct kvm_segment seg = {
		0, 				// base
		0xffffffff,		// limit
		0,				// selector
		11,             // type
		1,              // present
		0,              // dpl
		0,              // db
		1,              // s
		1,              // l
		1,              // g
		0,              // avl
		0,              // unusable
		0,              // padding
	};

	sregs->cs = seg;

	seg.type = 3; // Data: read, write, accessed
	sregs->ds = sregs->es = sregs->fs = sregs->gs = sregs->ss = seg;
}

void __vm::setup_long_mode(__vm &vm,kvm_sregs *sregs) {

    uint64_t mem_size = vm.mem_size;
    uint64_t page_size = vm.page_size;

	uint64_t page = 0;
	uint64_t pml4_addr = 0x1000; // Adrese su proizvoljne.
	uint64_t *pml4 = (uint64_t *)(vm.mem + pml4_addr);

	uint64_t pdpt_addr = 0x2000;
	uint64_t *pdpt = (uint64_t *)(vm.mem + pdpt_addr);

	uint64_t pd_addr = 0x3000;
	uint64_t *pd = (uint64_t *)(vm.mem + pd_addr);

	uint64_t pt_addr = 0x4000;
	uint64_t *pt = (uint64_t *)(vm.mem + pt_addr);

	pml4[0] = PDE64_PRESENT | PDE64_RW | PDE64_USER | pdpt_addr;
	pdpt[0] = PDE64_PRESENT | PDE64_RW | PDE64_USER | pd_addr;
	
    if(page_size == 2 * 1024 * 1024){

        uint64_t num_entries = mem_size / (2 * 1024 * 1024); // Calculate the number of entries based on mem_size
        for (int i = 0; i < num_entries && i < 4; i++) {
            pd[i] = PDE64_PRESENT | PDE64_RW | PDE64_USER | PDE64_PS | page;
            page += 2 << 20; // Increment page by 2MB
        }

    } else if(page_size == 4 * 1024){


        uint64_t number_of_2mbs = mem_size / (2 * 1024 * 1024);

        for(uint64_t i = 0; i < number_of_2mbs; ++i) {

            if(i != 0) {
                pt_addr += 0x1000;
                pt = (uint64_t *)(vm.mem + pt_addr);
            }

            pd[i] = PDE64_PRESENT | PDE64_RW | PDE64_USER | pt_addr;
            for(int j = 0; j < 512; ++j) {
                pt[j] = page | PDE64_PRESENT | PDE64_RW | PDE64_USER;
                page += 0x1000;
		    }

        }
    }

    // Registar koji ukazuje na PML4 tabelu stranica. Odavde kreće mapiranje VA u PA.
	sregs->cr3  = pml4_addr; 
	sregs->cr4  = CR4_PAE; // "Physical Address Extension" mora biti 1 za long mode.
	sregs->cr0  = CR0_PE | CR0_PG; // Postavljanje "Protected Mode" i "Paging" 
	sregs->efer = EFER_LME | EFER_LMA; // Postavljanje  "Long Mode Active" i "Long Mode Enable"

	// Inicijalizacija segmenata procesora.
	__vm::setup_64bit_code_segment(sregs);
}

int __vm::vm_body(std::string guest_image, uint64_t mem_size, uint64_t page_size) {
   
    __vm vm;
	vm.id = ++__vm::next_id;

	struct kvm_sregs sregs;
	struct kvm_regs regs;
	int stop = 0;
	int ret = 0;
	FILE* img;
    
	if (__vm::init_vm(vm, mem_size, page_size)) {
		printf("Failed to init the VM\n");
		return -1;
	}


	if (ioctl(vm.vcpu_fd, KVM_GET_SREGS, &sregs) < 0) {
		perror("KVM_GET_SREGS");
		return -1;
	}

	__vm::setup_long_mode(vm, &sregs);

    if (ioctl(vm.vcpu_fd, KVM_SET_SREGS, &sregs) < 0) {
		perror("KVM_SET_SREGS");
		return -1;
	}

	memset(&regs, 0, sizeof(regs));
	regs.rflags = 2;
	regs.rip = 0;
	// SP raste nadole

	regs.rsp = mem_size;

	if (ioctl(vm.vcpu_fd, KVM_SET_REGS, &regs) < 0) {
		perror("KVM_SET_REGS");
		return -1;
	}

    // NEW NEW NEW
	img = fopen(guest_image.c_str(), "r");
    // NEW NEW NEW
	if (img == NULL) {
		printf("Can not open binary file\n");
		return -1;
	}

	char *p = vm.mem;
  	while(feof(img) == 0) {
    	int r = fread(p, 1, 1024, img);
    	p += r;
  	}
  	fclose(img);

	while(stop == 0) {
		ret = ioctl(vm.vcpu_fd, KVM_RUN, 0);
		if (ret == -1) {
		printf("KVM_RUN failed\n");
		return 1;
		}

		switch (vm.kvm_run->exit_reason) {
			case KVM_EXIT_IO:
			{	

				// This is for working with console
				// Simple right?!
				char *p = (char *)vm.kvm_run + vm.kvm_run->io.data_offset ;

				if (vm.kvm_run->io.direction == KVM_EXIT_IO_OUT && vm.kvm_run->io.port == 0xE9) {

					std::cout << *p;

					// printf("%c", *p);
				} else if (vm.kvm_run->io.direction == KVM_EXIT_IO_IN && vm.kvm_run->io.port == 0xE9) {
					*p = getchar();
				}

				// DOOM music
				// Working with files

				// FILES
				if (vm.kvm_run->io.direction == KVM_EXIT_IO_OUT && vm.kvm_run->io.port == 0x0278) {
					
					// fopen
					// fclose
					// fwrite
					// fclose

					File_Controller &fc = vm.file_controller;

					// Skip until you get full msg
					if(fc.gather_msg(*p)) {
						continue;
					}

					switch(fc.get_operation()) 
					{
						case File_Controller::_fopen:
						{

							uint64_t ret = fc.file_open(vm.id);

							// Setup for sending
							fc.num_bool = true ;
							fc.return_number = ret;
							fc.num_cnt = 0;

							break;
						}
						case File_Controller::_fread:
						{

							std::string data = fc.file_read(vm.id);

							// Setup for sending
							fc.num_bool = true ;
							fc.return_number = data.size();
							fc.num_cnt = 0;
						
							fc.data_bool = true;
							fc.return_data = data;
							fc.data_cnt = 0;

							break;
						}
						case File_Controller::_fwrite:
						{
							fc.file_write(vm.id);
							break;
						}
						case File_Controller::_fclose:
						{
							fc.file_close(vm.id);
							
							break;
						}
						default:
						{
							printf("This is an error im vm get operation default OUT");
							exit(-1);
						}
					}


		
				} else if (vm.kvm_run->io.direction == KVM_EXIT_IO_IN && vm.kvm_run->io.port == 0x0278) {
					

					File_Controller& fc = vm.file_controller;


					if(fc.num_bool) {

						if(fc.num_cnt == 7) {
							fc.num_bool = false;
						}	

						*p = (uint8_t)(fc.return_number & 0xff);
						fc.return_number >>= 8;				

						fc.num_cnt += 1;

					} else if(fc.data_bool) {

						if(fc.data_cnt == fc.return_data.size() - 1) {
							fc.data_bool = false;
						}

						*p = fc.return_data[fc.data_cnt];

						fc.data_cnt += 1;
					}

				}
				continue;
			}
			case KVM_EXIT_HLT:
			{
				printf("KVM_EXIT_HLT\n");
				stop = 1;
				break;
			}
			case KVM_EXIT_INTERNAL_ERROR:
			{
				printf("Internal error: suberror = 0x%x\n", vm.kvm_run->internal.suberror);
				stop = 1;
				break;
			}
			case KVM_EXIT_SHUTDOWN:
			{
				printf("Shutdown\n");
				stop = 1;
				break;
			}
			default:
			{
				printf("Exit reason: %d\n", vm.kvm_run->exit_reason);
				break;
			}
    	}
  	}

	return 0;
}

__vm::~__vm() {}