# Testio

We have all written some form of library to wrap calls to basic C IO functions (`read`, `write`, `send`, `recv`) with nicer APIs - usally adding suffixes such as `_all` or `_exact`.

Some of us even had the pleasure to write these functions more than once.

Testio tries to create a simple way to test those libraries.

## Requirements
* python 3
* pytest
* fuse3

## Usage

* Write your IO library, make sure it implements the `test.h` header present in the root directory of this project.

* Compile your library as a shared object (for Linux/GNU x86_64)

* Build the testio project (or use the binaries in the GitHub releases page)

* Run `./test.py --lib <path to your lib>`

An example library is found under `example`, you can compile it using `make`.

You can also pass the following flags to `test.py`:

* `--verbose-fuse` - Increase log verbosity of the testfs fuse
* `--verbose-tester` - Increase log verbosity of the low-level tester
* `--fuse-bin=FUSE_BIN` - Path to the testfs fuse binary (`./target/debug/testio` by default)
* `--tester-bin=TESTER_BIN` - Path to the tester binary (`./target/debug/tester` by default)
* `--lib=LIB` - Path to the library that will be tested (`./example/libexample.so` by default)
* any other flag accepted by `pytest`

## Building
Building this project requires a Rust toolchain.

To build run:

    cargo build

To run the testfs fuse without the tester for debugging:

    cargo run --bin testio <mount path>

## How does it work

Testio creates a FUSE filesystem, that pits the tested library against various edge cases that can happen
during `read` / `write` - such as incomplete `read` / `write` calls.

The filesystem defines files with different `read` / `write` handler functions (`src/bin/testio.rs:create_files`) that affect the result of each IO call.

For reading, the API allows specifying a lambda that receives the `count` of the current call to `read`, and returns a new `count` that will be used instead.

For writing, the API allows specifying a lambda that receives the array of bytes of the current call to `write`, and returns a new array that will be written instead.