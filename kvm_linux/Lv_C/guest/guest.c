#include <stddef.h>
#include <stdint.h>

static uint16_t console_port = 0x0E9;

static void outb(uint16_t port, uint8_t value) {
	asm("outb %0,%1" : /* empty */ : "a" (value), "Nd" (port) : "memory");
}

static uint8_t inb(uint16_t port) {
    uint8_t ret;
    asm volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

// Function to send a 64-bit value to a specified port by splitting it into bytes
void outb64(uint16_t port, uint64_t value) {
	
    for (int i = 0; i < 8; ++i) {
        outb(port, (uint8_t)(value >> (i * 8)));
    }
}

// Function to read a 64-bit value from a specified port by combining bytes
uint64_t inb64(uint16_t port) {
    uint64_t value = 0;
    for (int i = 0; i < 8; ++i) {
        value |= (uint8_t)(inb(port)) << (i * 8);
    }
    return value;
}

static uint16_t file_port = 0x0278;

static uint64_t fopen(const char* file_name) {

	outb(file_port, 0x01);
	outb(file_port, '#');


	for(int i = 0; file_name[i] != '\0'; i++) {
		outb(file_port, file_name[i]);
	}

	outb(file_port, '#');
	outb(file_port, 'w');
	outb(file_port, '#');
	outb(file_port, '#');

	uint64_t ret = inb64(file_port);

	return ret;
}

static int fread(const uint64_t FILE, char* buffer, uint64_t size) {

	outb(file_port, 0x02);
	outb(file_port, '#');

	outb64(file_port, FILE);
	outb(file_port, '#');

	outb64(file_port, size);
	outb(file_port, '#');
	outb(file_port, '#');

	// Getting the data

	uint64_t ret = inb64(file_port);

	for(int i = 0; i < ret; i++) {
		buffer[i] = inb(file_port);
	}

	return ret;
}

static int fwrite(const uint64_t FILE, char* buffer, uint64_t size) {

	outb(file_port, 0x03);
	outb(file_port, '#');

	outb64(file_port, FILE);
	outb(file_port, '#');

	for(int i = 0; i < size; i++) {
		outb(file_port, buffer[i]);
	}

	outb(file_port, '#');
	outb(file_port, '#');

	return 0;
}

static int fclose(const uint64_t FILE) {

	outb(file_port, 0x04);
	outb(file_port, '#');

	outb64(file_port, FILE);
	outb(file_port, '#');

	outb(file_port, '#');
	outb(file_port, '#');

	return 0;
}

void test_1() {
	uint64_t file_id = fopen("flowers.txt");

	char buffer[256];
	uint64_t ss = fread(file_id, buffer, 256);

	for(int i = 0; i < ss; i++) {
		outb(console_port, buffer[i]);
	}

	char buffer2[5] = {'L', 'A', 'Z', 'A', 'R'};

	fwrite(file_id, buffer2, 5);

	ss = fread(file_id, buffer, 256);

	for(int i = 0; i < ss; i++) {
		outb(console_port, buffer[i]);
	}


	fclose(file_id);
}

void test_2() {

	uint64_t simple_img = fopen("color_test.ppm");

	uint64_t header_size = 15;

	char *buffer ="P6\n250 250\n255\n";

	fwrite(simple_img, buffer, header_size);

	for(int r=0; r<253; r++) {
		for(int b=0; b<255; b++) {

		char single_buffer[3];


		// Guards for delimiters used in serialization
		if(r == '#') {
			r += 1;
		}

		if(b == '#') {
			b += 1;
		}

		single_buffer[0] = r;
		single_buffer[1] = 0;
		single_buffer[2] = b;
		fwrite(simple_img, single_buffer, 3);

		}
  	}
	fclose(simple_img);
}

void test_3() {

	uint64_t mul_wr = fopen("multiple_write.txt");

	char *buffer = "PETAK";
	fwrite(mul_wr, buffer, 5);

	char *buffer2 = " je idealan";

	fwrite(mul_wr, buffer2, 11);

	fclose(mul_wr);

}

void test_4() {


	uint64_t local_file = fopen("local_file_example.txt");

	char* buffer = "This is only a local file!";

	fwrite(local_file, buffer, 26);

	fclose(local_file);

}

void test_5() {

	uint64_t shared_file = fopen("to_duplicate.ppm");

	uint64_t img_copy = fopen("image_copy.ppm");

	uint64_t ss;
	while(1) {

		char buffer[256];
		ss = fread(shared_file, buffer, 256);

		if(ss == 0) {
			break;
		}

		fwrite(img_copy, buffer, ss);

	}

	fclose(shared_file);
	fclose(img_copy);

}

void
__attribute__((noreturn))
__attribute__((section(".start")))
_start(void) {

	/*
		INSERT CODE BELOW THIS LINE
	*/

	test_1(); // opens a shared file, reads from it to console, and writes something else

	test_2(); // Generates an image

	test_3(); // Multiple writes

	test_4(); // opens and writes to a local file

	test_5(); // image copy

	/*
		INSERT CODE ABOVE THIS LINE
	*/

	for (;;)
		asm("hlt");
}
