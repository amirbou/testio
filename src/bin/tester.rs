use clap::{Parser, Subcommand};
use libc::{ssize_t, size_t, c_int};
use std::{fs::File, ffi::{CString, OsStr, OsString}, os::unix::prelude::AsRawFd, io::{Seek, SeekFrom}};
#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct IOLibrary {
    read_all: extern "C" fn(fd: c_int, data: *mut u8, size: size_t) -> ssize_t,
    write_all: extern "C" fn(fd: c_int, data: *const u8, size: size_t) -> ssize_t
}

#[derive(Parser)]
#[clap(author, version, long_about = None)]
#[clap(about = "Tests a given r/w wrapper library")]
struct TesterCli {
    
    /// The path to the library
    library_path: OsString,
    
    /// The path to the file to read
    file_path: OsString,

    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Tests the read_all functionality of the library
    Read { 
        #[clap(long)]
        #[clap(default_value_t = 0)]
        /// The offset to start reading from
        offset: u64,
        #[clap(long)]
        /// The count to read. by default, the whole file is read
        count: Option<usize>,
    },

    /// Tests the write_all functionality of the library
    Write {
        /// The data to write
        data: String,
    }
}


fn load_library(library_path: &OsStr) -> Container<IOLibrary>
{

    unsafe { Container::load(library_path).expect("Failed to load the library!") }

}

fn open_file(file_path: &OsStr, for_write: bool) -> File {
    File::options().read(true).write(for_write).open(file_path).expect("Failed to open file!")
}

fn handle_read(library: Container<IOLibrary>, mut file: File, offset: u64, count: Option<usize>) -> ssize_t {
    let count = match count {
        Some(count) => count,
        None => file.metadata().expect("Failed to calculate file length").len() as usize,
    };

    if offset != 0
    {
        file.seek(SeekFrom::Start(offset as u64)).expect("Failed to set offset");
    }
    let mut buf= Vec::<u8>::with_capacity(count as usize);
    let buf_ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    let result = library.read_all(file.as_raw_fd(), buf_ptr, count as size_t);
    
    let result_length = std::cmp::max(result, 0) as usize;
    let buffer = unsafe {
        Vec::from_raw_parts(
            buf_ptr,
            result_length,
            count)
    };
    println!("{}", String::from_utf8(buffer).expect("Failed to decode data"));
    result
}

fn handle_write(library: Container<IOLibrary>, file: File, data: String) -> ssize_t {
    let data: CString = CString::new(data).unwrap();
    let bytes = data.as_bytes();

    let size = bytes.len();

    library.write_all(file.as_raw_fd(), bytes.as_ptr(), size)
}


fn main() {
    env_logger::init();

    let cli = TesterCli::parse();


    let library_path = &cli.library_path;
    let file_path = &cli.file_path;
    
    let library = load_library(library_path);

    let result = match cli.command {
        Commands::Read { offset, count } => handle_read(
            library,
            open_file(file_path, false),
            offset,
            count
        ),
        Commands::Write { data } => handle_write(
            library,
            open_file(file_path, true),
            data
        ),
    };

    println!("{}", result);

}
