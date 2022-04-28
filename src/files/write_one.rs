use fuser::FileAttr;

use crate::testfs::{FsFile, Result};
use crate::files::file_base::{ReadableFile,WriteableFile};

pub struct WriteOneFile {
    name: std::ffi::OsString,
    data: Vec<u8>
}

impl WriteOneFile {
    pub fn new(name: String) -> Self {
        
        Self { name: name.into(), data: Vec::new() }
    }
}

impl ReadableFile for WriteOneFile {
    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_perms(&self) -> u16 {
        0o666
    }
}

impl WriteableFile for WriteOneFile {
    fn get_data_mut(&mut self) -> &mut Vec<u8> { 
        &mut self.data
    }
}

impl FsFile for WriteOneFile {

    fn get_name(&self) -> &std::ffi::OsStr {
        &self.name
    }

    fn read(&self, offset: i64, size: u32, _flags: i32) -> Result<&[u8]> {
        Ok(self._read(offset, size))
    }

    fn getattr(&self) -> FileAttr {
        self._getattr()
    }

    fn write(&mut self, offset: i64, data: &[u8], _flags: i32) -> Result<u32> {
        Ok(self._write(offset, &data[..1]).try_into().unwrap())
    }

    fn setattr(
        &mut self,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        flags: Option<u32>,
    ) -> Result<FileAttr>{
        match self._setattr(mode, uid, gid, size, flags) {
            Some(attr) => Ok(attr),
            None => Err(libc::ENOSYS),
        }
    }
}