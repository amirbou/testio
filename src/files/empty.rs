use fuser::FileAttr;

use crate::testfs::{FsFile, Result};
use crate::files::file_base::ReadableFile;

const NO_DATA: [u8; 0] = [];

pub struct EmptyROFile {
    name: std::ffi::OsString,

}

impl EmptyROFile {
    pub fn new(name: String) -> Self {
        
        Self { name: name.into() }
    }
}

impl ReadableFile for EmptyROFile {
    fn get_data(&self) -> &[u8] {
        &NO_DATA
    }
}

impl FsFile for EmptyROFile {

    fn get_name(&self) -> &std::ffi::OsStr {
        &self.name
    }

    /*
     * Empty file - all reads return EOF
     */
    fn read(&self, offset: i64, size: u32, _flags: i32) -> Result<&[u8]> {
        Ok(self._read(offset, size))
    }

    fn getattr(&self) -> FileAttr {
        self._getattr()
    }

    fn write(&mut self, _offset: i64, _data: &[u8], _flags: i32) -> Result<u32> {
        Err(libc::ENOSYS)
    }
}