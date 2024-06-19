CC = gcc 
GG = g++ -std=c++17

SRC_FILES = $(wildcard ./src/*.cpp)
GST_FILES = $(wildcard ./guest/*.c)
OBJ_FILES = $(patsubst ./guest/%.c, ./guest/%.o, $(GST_FILES))
IMG_FILES = $(patsubst ./guest/%.c, %.img, $(GST_FILES))

CFLAGS = -Iinc -I.

all: mini_hypervisor $(IMG_FILES)

mini_hypervisor: $(SRC_FILES)
	@echo "Building mini_hypervisor"
	$(GG) $(CFLAGS) $^ -o $@

%.img: guest/%.o
	@echo "Linking $@"
	ld -T guest/guest.ld $< -o $@

guest/%.o: guest/%.c
	@echo "Compiling $<"
	$(CC) -m64 -ffreestanding -fno-pic -c -o $@ $<

clean:
	rm -f $(OBJ_FILES) $(IMG_FILES)
	rm -f mini_hypervisor