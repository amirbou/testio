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
    
    fn get_perms(&self) -> u16 {
        return 0o444;
    }

    fn _getattr(&self) -> FileAttr {
        generate_fileattr(self.get_size() as u64, self.get_perms(), false)
    }
}

pub trait WriteableFile: ReadableFile {
    fn get_data_mut(&mut self) -> &mut Vec<u8>;

    fn _write(&mut self, offset: i64, new_data: &[u8]) -> usize
    {
        let data = self.get_data_mut();
        let offset = offset as usize;
        
        // create a "hole" if needed
        if offset > data.len() {
            data.resize(offset, 0);
        }

        // append
        if offset >= data.len() {
            data.extend_from_slice(new_data);
            return new_data.len();
        }

        // data:     XXXXXXXXXX
        // new_data:       YYYYYYYYY
        // result:   XXXXXXYYYYYYYYY
        if offset + new_data.len() > data.len() {
            data.resize(offset, 0);
            data.extend_from_slice(new_data);
            return new_data.len();
        }
        
        // data:     XXXXXXXXXX
        // new_data:   YYYY
        // result:   XXYYYYXXXX
        
        let mut end = data.split_off(offset);
        data.extend(new_data);
        end.drain(0..new_data.len());
        data.extend(end);

        new_data.len()
    }

    // setattr with size = 0 is called when the file is truncated (like when using bash >)
    fn _setattr(
        &mut self,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        flags: Option<u32>,
    ) -> Option<FileAttr> {
        if mode.is_some() || uid.is_some() || gid.is_some() || flags.is_some() {
            return None
        }
        if let Some(size) = size {
            self.get_data_mut().resize(size as usize, 0);
        }
        
        Some(generate_fileattr(self.get_size() as u64, self.get_perms(), false))
    }
}