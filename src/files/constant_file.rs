use fuser::FileAttr;

use crate::testfs::generate_fileattr;

pub trait ReadableFile {
    fn get_data(&self) -> &[u8];
    
    fn get_size(&self) -> usize {
        self.get_data().len()
    }
    
    fn _read(&self, offset: i64, size: u32) -> &[u8]{
        let data = self.get_data();
        let mut offset = offset as usize;

        if offset > data.len() {
            offset = data.len();
        }
        let mut end = offset + size as usize;
        if end > data.len() {
            end = data.len();
        }
        &data[offset..end]
    }

    fn _getattr(&self) -> FileAttr {
        generate_fileattr(self.get_size() as u64, 0o444, false)
    }
}
