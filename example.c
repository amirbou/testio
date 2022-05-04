// compile with -shared
#include <unistd.h>
#include <sys/types.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>


ssize_t read_all(int fd, uint8_t * buf, size_t buf_size) {
    ssize_t current_read = 0;
    ssize_t total_read = 0;
    do {
        current_read = read(fd, buf, buf_size);
        if (0 > current_read)
        {
            perror("read");
            return -1;
        }
        buf += current_read;
        buf_size -= current_read;
        total_read += current_read;
    } while (current_read > 0 && buf_size >= 0);
    
    if (buf_size < 0)
    {
        fprintf(stderr, "file is bigger then buffer!\n");
        return -1;
    }

    return total_read;

}


ssize_t write_all(int fd, const uint8_t * buf, size_t buf_size) {
    ssize_t current_write = 0;
    ssize_t total_write = 0;
    while (buf_size > 0)
    {
        current_write = write(fd, buf, buf_size);
        if (0 > current_write)
        {
            perror("write");
            return -1;
        }
        buf += current_write;
        buf_size -= current_write;
        total_write += current_write;
    }

    return total_write;
}
