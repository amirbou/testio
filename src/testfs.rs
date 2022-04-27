use fuser::{self, Request, ReplyAttr, FileAttr, ReplyOpen, FileType};
use fuser::Filesystem;
use libc;
use log::debug;
use std::option::Option;
use std::time::{Duration, UNIX_EPOCH};

pub type Result<T> = std::result::Result<T, libc::c_int>;

const INODE_BIAS: u64 = 2;
const ROOT_INODE: u64 = 1;
const TTL: Duration = Duration::from_secs(1);
pub trait FsFile
{
    fn get_name(&self) -> &std::ffi::OsStr;
    fn read(&self, offset: i64, size: u32, flags: i32) -> Result<&[u8]>;
    fn write(&mut self, offset: i64, data: &[u8], flags: i32) -> Result<u32>;
    fn getattr(&self) -> FileAttr;

    fn open(&self, _flags:  i32) -> Result<(u64, u32)> {
        Ok((0, fuser::consts::FOPEN_DIRECT_IO))
    }

    fn release(&self, _flags: i32, _flush: bool) ->  Result<()> {
        Ok(())
    }

    fn setattr(
        &mut self,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        flags: Option<u32>,
    ) -> Result<FileAttr>{
        debug!(
            "[Not Implemented] setattr(mode: {:?}, uid: {:?}, \
            gid: {:?}, size: {:?}, flags: {:?}) on file {:?}",
            mode, uid, gid, size, flags, self.get_name()
        );
        Err(libc::ENOSYS)
    }
}

pub fn generate_fileattr(size: u64, perm: u16, is_dir: bool) -> FileAttr {
    let blocks = match size {
        0 => 0,
        _ => 8,
    };
    
    let kind = match is_dir {
        true => fuser::FileType::Directory,
        false => fuser::FileType::RegularFile,
    };

    let attr = FileAttr {
        ino: 0,
        size,
        blocks,
        atime: UNIX_EPOCH,
        mtime: UNIX_EPOCH,
        ctime: UNIX_EPOCH,
        crtime: UNIX_EPOCH,
        kind,
        perm,
        nlink: 1,
        uid: 1000,
        gid: 1000,
        rdev: 0,
        blksize: 4096,
        flags: 0
    };
    attr
}

fn ino_to_idx(ino: u64) -> usize {
    (ino - INODE_BIAS) as usize
}

fn idx_to_ino(idx: usize) -> u64 {
    idx as u64 + INODE_BIAS
}

pub struct TestFs {
    files: Vec<Box<dyn FsFile>>,
}

impl TestFs {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn add_file(&mut self, file: Box<dyn FsFile>) {
        self.files.push(file);
    }

    fn get_file(&self, ino: u64) -> Option<&Box<dyn FsFile>>{
        let idx = ino_to_idx(ino);
        self.files.get(idx)
    }

    fn get_file_mut(&mut self, ino: u64) -> Option<&mut Box<dyn FsFile>>
    {
        let idx = ino_to_idx(ino);
        self.files.get_mut(idx)
    }
}

impl Filesystem for TestFs {
    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: ReplyAttr)
    {
        // root '/' dir
        if ino == ROOT_INODE {
            let mut attr = generate_fileattr(4096, 0o755, true);
            attr.ino = ino;
            reply.attr(&TTL, &attr);
        }
        else {
            let file = match self.get_file(ino) {
                Some(file) => file,
                None => return reply.error(libc::ENOENT),
            };
            let attr = {
                let mut attr = file.getattr();
                attr.ino = ino;
                attr
            };
            reply.attr(&TTL, &attr);
        }
    }

    fn init(&mut self, _req: &Request<'_>, _config: &mut fuser::KernelConfig) -> Result<()> {
        Ok(())
    }

    fn destroy(&mut self) {}

    fn lookup(&mut self, _req: &Request<'_>, parent: u64, name: &std::ffi::OsStr, reply: fuser::ReplyEntry) {
        debug!(
            "lookup(parent: {:#x?}, name {:?})",
            parent, name
        );
        if parent != ROOT_INODE {
            reply.error(libc::EBADF);
            return;
        }
        for (idx, file) in self.files.iter().enumerate() {
            if file.get_name() == name {
                let attr = {
                    let mut attr = file.getattr();
                    attr.ino = idx_to_ino(idx);
                    attr
                };
                reply.entry(&TTL, &attr, 1);
                return;
            }
        }
        reply.error(libc::ENOENT);
    }

    fn forget(&mut self, _req: &Request<'_>, _ino: u64, _nlookup: u64) {}

    fn setattr(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        _atime: Option<fuser::TimeOrNow>,
        _mtime: Option<fuser::TimeOrNow>,
        _ctime: Option<std::time::SystemTime>,
        _fh: Option<u64>,
        _crtime: Option<std::time::SystemTime>,
        _chgtime: Option<std::time::SystemTime>,
        _bkuptime: Option<std::time::SystemTime>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        let file = match self.get_file_mut(ino) {
            Some(file) => file,
            None => return reply.error(libc::ENOENT)
        };
        match file.setattr(mode, uid, gid, size, flags) {
            Ok(attr) => reply.attr(&TTL, &attr),
            Err(err) => reply.error(err),
        }
    }

    fn unlink(&mut self, _req: &Request<'_>, parent: u64, name: &std::ffi::OsStr, reply: fuser::ReplyEmpty) {
        debug!(
            "[Not Implemented] unlink(parent: {:#x?}, name: {:?})",
            parent, name,
        );
        reply.error(libc::ENOSYS);
    }

    fn open(&mut self, _req: &Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        let file = match self.get_file(ino) {
            Some(file) => file,
            None => return reply.error(libc::ENOENT)
        };
        match file.open(flags) {
            Ok((fh, flags)) => reply.opened(fh, flags),
            Err(err) => reply.error(err)
        };
    }
    
    fn opendir(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _flags: i32,
        reply: ReplyOpen
    ) {
        reply.opened(0, 0);
    }

    fn read(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        debug!(
            "read(ino: {:#x?}, fh: {}, offset: {}, size: {}, \
            flags: {:#x?}, lock_owner: {:?})",
            ino, fh, offset, size, flags, lock_owner
        );

        let file = match self.get_file(ino) {
            Some(file) => file,
            None => return reply.error(libc::ENOENT),
        };
        match file.read(offset, size, flags) {
            Ok(data) => reply.data(data),
            Err(err) => reply.error(err),
        };
    }

    fn write(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        write_flags: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: fuser::ReplyWrite,
    ) {
        debug!(
            "write(ino: {:#x?}, fh: {}, offset: {}, data.len(): {}, \
            write_flags: {:#x?}, flags: {:#x?}, lock_owner: {:?})",
            ino,
            fh,
            offset,
            data.len(),
            write_flags,
            flags,
            lock_owner
        );
        let file = match self.get_file_mut(ino) {
            Some(file) => file,
            None => return reply.error(libc::ENOENT),
        };
        match file.write(offset, data, flags) {
            Ok(size) => reply.written(size),
            Err(err) => reply.error(err),
        };
    }

    fn flush(&mut self, _req: &Request<'_>, ino: u64, fh: u64, lock_owner: u64, reply: fuser::ReplyEmpty) {
        debug!(
            "flush(ino: {:#x?}, fh: {}, lock_owner: {:?})",
            ino, fh, lock_owner
        );
        reply.ok();
    }

    fn release(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        flags: i32,
        _lock_owner: Option<u64>,
        flush: bool,
        reply: fuser::ReplyEmpty,
    ) {
        let file = match self.get_file(ino) {
            Some(file) => file,
            None => return reply.error(libc::ENOENT)
        };
        match file.release(flags, flush) {
            Ok(()) => reply.ok(),
            Err(err) => reply.error(err)
        };
    }

    fn fsync(&mut self, _req: &Request<'_>, ino: u64, fh: u64, datasync: bool, reply: fuser::ReplyEmpty) {
        debug!(
            "fsync(ino: {:#x?}, fh: {}, datasync: {})",
            ino, fh, datasync
        );
        reply.ok()
    }

    fn readdir(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: fuser::ReplyDirectory,
    ) {
        debug!(
            "readdir(ino: {:#x?}, fh: {}, offset: {})",
            ino, fh, offset
        );
        if ino != ROOT_INODE {
            reply.error(libc::ENOENT);
            return;
        }
        let count = self.files.len();
        // if offset == count, we should return EOF - fallthrough to the loop which will return an empty reply
        if offset < 0 || offset as usize > count {
            reply.error(libc::EINVAL);
            return;
        }
        let offset = offset as usize;
        
        let iter = self.files.iter().enumerate().skip(offset);
        for (idx, file) in iter {
            let ino = idx_to_ino(idx);
            let name = file.get_name();
            // offset is the index of the next entry
            let offset = idx as i64 + 1;
            let kind = FileType::RegularFile;
            if !reply.add(
                ino,
                offset,
                kind,
                name
            ) {
                break;
            }
        }
        reply.ok();

    }


    fn access(&mut self, _req: &Request<'_>, ino: u64, mask: i32, reply: fuser::ReplyEmpty) {
        debug!("[Not Implemented] access(ino: {:#x?}, mask: {})", ino, mask);
        reply.error(libc::ENOSYS);
    }

    fn lseek(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        whence: i32,
        reply: fuser::ReplyLseek,
    ) {
        debug!(
            "[Not Implemented] lseek(ino: {:#x?}, fh: {}, offset: {}, whence: {})",
            ino, fh, offset, whence
        );
        reply.error(libc::ENOSYS);
    }
}