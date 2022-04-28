use fuser::FileAttr;

use crate::testfs::{FsFile, Result};
use crate::files::file_base::{ReadableFile};

pub struct ReadOneFile {
    name: std::ffi::OsString,
    data: Vec<u8>
}

impl ReadOneFile {
    pub fn new(name: String, data: String) -> Self {
        
        Self { name: name.into(), data: data.into() }
    }
}

impl ReadableFile for ReadOneFile {
    fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl FsFile for ReadOneFile {

    fn get_name(&self) -> &std::ffi::OsStr {
        &self.name
    }

    fn read(&self, offset: i64, _size: u32, _flags: i32) -> Result<&[u8]> {
        Ok(self._read(offset, 1))
    }

    fn getattr(&self) -> FileAttr {
        self._getattr()
    }

    fn write(&mut self, _offset: i64, _data: &[u8], _flags: i32) -> Result<u32> {
        Err(libc::ENOSYS)
    }
}