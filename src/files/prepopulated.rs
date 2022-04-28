use fuser::FileAttr;

use crate::testfs::{FsFile, Result};
use crate::files::file_base::ReadableFile;

pub struct PrepopulatedFile {
    name: std::ffi::OsString,
    data: Vec<u8>
}

impl PrepopulatedFile {
    pub fn new(name: String, data: String) -> Self {
        
        Self { name: name.into(), data: data.into() }
    }
}

impl ReadableFile for PrepopulatedFile {
    fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl FsFile for PrepopulatedFile {

    fn get_name(&self) -> &std::ffi::OsStr {
        &self.name
    }

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