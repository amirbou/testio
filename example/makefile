.PHONY: all clean

CC := gcc
FLAGS := -shared -I..
TARGET := libexample.so

all: $(TARGET)

$(TARGET): example.c
	$(CC) $< -o $@ $(FLAGS)

clean:
	rm -f $(TARGET)