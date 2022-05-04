use fuser::FileAttr;

use crate::testfs::{FsFile, Result};
use crate::files::file_base::{ReadableFile};

pub struct ReadX<F: Fn(u32,) -> u32> {
    name: std::ffi::OsString,
    data: Vec<u8>,
    read_size_func: F
}

impl<F: Fn(u32,) -> u32> ReadX<F>
{
    pub fn new(name: String, data: String, read_size_func: F) -> Self {
        
        Self { name: name.into(), data: data.into(), read_size_func }
    }
}

impl<F: Fn(u32,) -> u32> ReadableFile for ReadX<F> {
    fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl<F: Fn(u32,) -> u32> FsFile for ReadX<F> {

    fn get_name(&self) -> &std::ffi::OsStr {
        &self.name
    }

    fn read(&self, offset: i64, size: u32, _flags: i32) -> Result<&[u8]> {
        let size = match size {
            0 => 0,
            _ => (self.read_size_func)(size)
        };
        Ok(self._read(offset, size))
    }

    fn getattr(&self) -> FileAttr {
        self._getattr()
    }

    fn write(&mut self, _offset: i64, _data: &[u8], _flags: i32) -> Result<u32> {
        Err(libc::ENOSYS)
    }
}