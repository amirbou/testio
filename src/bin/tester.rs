use clap::{arg, Command, ArgMatches};
use libc::{ssize_t, size_t, c_int};
use std::{fs::File, ffi::CString, os::unix::prelude::AsRawFd};
#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct IOLibrary {
    read_all: extern "C" fn(fd: c_int, data: *mut u8, size: size_t) -> ssize_t,
    write_all: extern "C" fn(fd: c_int, data: *const u8, size: size_t) -> ssize_t
}

fn load_library(library_path: String) -> Container<IOLibrary>
{

    unsafe { Container::load(library_path).expect("Failed to load the library!") }

}

fn open_file(file_path: String, for_write: bool) -> File {
    File::options().read(true).write(for_write).open(file_path).expect("Failed to open file!")
}

fn handle_read(library: Container<IOLibrary>, file: File, _sub_matches: &ArgMatches) -> ssize_t {
    let file_len = file.metadata().expect("Failed to calculate file length").len() as usize;
    let mut buf= Vec::<u8>::with_capacity(file_len as usize);
    let buf_ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    let result = library.read_all(file.as_raw_fd(), buf_ptr, file_len as size_t);
    
    let result_length = std::cmp::max(result, 0) as usize;
    let buffer = unsafe {
        Vec::from_raw_parts(
            buf_ptr,
            result_length,
            file_len)
    };
    println!("{}", String::from_utf8(buffer).expect("Failed to decode data"));
    result
}

fn handle_write(library: Container<IOLibrary>, file: File, sub_matches: &ArgMatches) -> ssize_t {
    let data: String = sub_matches.value_of("data").expect("required").into();
    let data: CString = CString::new(data).unwrap();
    let bytes = data.as_bytes();

    let size = bytes.len();

    library.write_all(file.as_raw_fd(), bytes.as_ptr(), size)
}


fn main() {
    env_logger::init();

    // todo: add a feature to seek the file before calling functions
    let matches = Command::new("Tester")
        .version("wip")
        .author("AmirB")
        .about("Tests a given r/w wrapper library")
        .arg(arg!(<library_path> "The path to the library"))
        .arg(arg!(<file_path> "The path to the file to read"))
        .subcommand_required(true)
        .subcommand(
            Command::new("read")
                .about("calls the read_all function of the library")
        )
        .subcommand(
            Command::new("write")
                .about("calls the write_all function of the libraray")
                .arg(arg!(<data> "The data to write"))
        )
        .get_matches();

    let library_path: String = matches.value_of("library_path").expect("required").into();
    let file_path: String = matches.value_of("file_path").expect("required").into();
    
    let library = load_library(library_path);

    let result = match matches.subcommand() {
        Some(("read", sub_matches)) => handle_read(library, open_file(file_path, false), sub_matches),
        Some(("write", sub_matches)) => handle_write(library, open_file(file_path, true), sub_matches),
        _ => unreachable!()
    };

    println!("{}", result);

}
