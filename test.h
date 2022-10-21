#include <sys/types.h>
#include <inttypes.h>

/**
 * This function reads `buf_size` bytes from file `fd` to the buffer `buf`
 * 
 * @param fd - the file to read from
 * @param buf - the buffer to read into - must be large enought to hold `buf_size` bytes
 * @param buf_size - the amount of bytes to read
 * 
 * @returns buf_size on success, -1 on error.
 *           Failing to read exactly buf_size bytes (for example - if the file is shorter) is considered an error
 */
ssize_t read_all(int fd, void * buf, size_t buf_size);

/**
 * This function writes `buf_size` bytes from buffer `buf` to file `fd`
 * 
 * @param fd - the file to write to
 * @param buf - the buffer to write from - must contain at least `buf_size` bytes
 * @param buf_size - the amount of bytes to write
 * 
 * @returns buf_size on success, -1 on error.
 *           Failing to write exactly buf_size bytes is considered an error
 */
ssize_t write_all(int fd, const void * buf, size_t buf_size);
