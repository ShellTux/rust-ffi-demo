CC = clang

CFLAGS += -Wall -Wextra -Werror
CFLAGS += -Iinclude

# Source and object files
SRC = src/lib.c
OBJ = lib/lib.o

# Output library
TARGET = lib/liblib.a

# Default target
all: $(TARGET)

$(OBJ): $(SRC)
	mkdir --parents $(shell dirname $@)
	$(CC) $(CFLAGS) -c $^ -o $@

$(TARGET): $(OBJ)
	ar rcs $(TARGET) $(OBJ)

clean:
	rm -f $(OBJ) $(TARGET)

.PHONY: all clean
