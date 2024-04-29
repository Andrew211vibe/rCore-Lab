use super::{File, StatMode};
use crate::drivers::BLOCK_DEVICE;
use crate::mm::UserBuffer;
use crate::sync::UPSafeCell;
use alloc::{string::String, sync::Arc};
use alloc::vec::Vec;
use bitflags::*;
use easy_fs::{EasyFileSystem, Inode};
use lazy_static::*;

/// inode in memory
/// A wrapper around a filesystem inode
/// to implement File trait atop
pub struct OSInode {
    readable: bool,
    writeable: bool,
    inner: UPSafeCell<OSInodeInner>,
}

/// The OS inode inner in 'UPSafeCell'
pub struct OSInodeInner {
    offset: usize,
    inode: Arc<Inode>,
}

impl OSInode {
    pub fn new(readable: bool, writeable: bool, inode: Arc<Inode>) -> Self {
        Self {
            readable,
            writeable,
            inner: unsafe { UPSafeCell::new(OSInodeInner { offset: 0, inode }) },
        }
    }

    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.exclusive_access();
        let mut buffer = [0_u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            if len == 0 {
                break;
            }
            inner.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }

    pub fn get_inode_stat(&self) -> StatMode {
        let inner = self.inner.exclusive_access();
        StatMode::from_bits(inner.inode.get_inode_stat()).unwrap()
    }

    pub fn get_inode_id(&self) -> u64 {
        let inner = self.inner.exclusive_access();
        inner.inode.get_inode_id()
    }
}

lazy_static! {
    pub static ref ROOT_INODE: Arc<Inode> = {
        let efs = EasyFileSystem::open(BLOCK_DEVICE.clone());
        Arc::new(EasyFileSystem::root_inode(&efs))
    };
}

pub fn list_apps() {
    println!("/**** APPS ****");
    for app in ROOT_INODE.ls() {
        println!("{}", app);
    }
    println!("**************/");
}

bitflags! {
    /// The flags argument to the open() system call is constructed by ORing together zero or moreof the following values:
    pub struct OpenFlags: u32 {
        /// read only
        const RDONLY = 0;
        /// write only
        const WRONLY = 1 << 0;
        /// read and write
        const RDWR = 1 << 1;
        /// create new file
        const CREATE = 1 << 9;
        /// truncate file size to 0
        const TRUNC = 1 << 10;
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writeable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

pub fn open_file(name: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
    let (readable, writeable) = flags.read_write();
    if flags.contains(OpenFlags::CREATE) {
        if let Some(inode) = ROOT_INODE.find(name) {
            // clear size
            inode.clear();
            Some(Arc::new(OSInode::new(readable, writeable, inode)))
        } else {
            // create file
            ROOT_INODE
                .create(name)
                .map(|inode| Arc::new(OSInode::new(readable, writeable, inode)))
        }
    } else {
        ROOT_INODE.find(name).map(|inode| {
            if flags.contains(OpenFlags::TRUNC) {
                inode.clear();
            }
            Arc::new(OSInode::new(readable, writeable, inode))
        })
    }
}

impl File for OSInode {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writeable(&self) -> bool {
        self.writeable
    }
    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        let mut total_read_size = 0_usize;
        for slice in buf.buffers.iter_mut() {
            let read_size = inner.inode.read_at(inner.offset, *slice);
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
            total_read_size += read_size;
        }
        total_read_size
    }
    fn write(&self, buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        let mut total_write_size = 0_usize;
        for slice in buf.buffers.iter() {
            let write_size = inner.inode.write_at(inner.offset, *slice);
            assert_eq!(write_size, slice.len());
            inner.offset += write_size;
            total_write_size += write_size;
        }
        total_write_size
    }
    fn fstat(&self) -> (u64, StatMode, u32) {
        let ino = self.get_inode_id();
        let mode = self.get_inode_stat();
        let nlink = ROOT_INODE.get_link_count(ino);
        (ino, mode, nlink)
    }
}

pub fn linkat(old_name: *const u8, new_name: *const u8) -> isize {
    let mut length = 0;
    unsafe {
        let mut ptr = old_name;
        while (*ptr) as char != '\0' {
            length += 1;
            ptr = ptr.offset(1);
        }
        let old_name = core::str::from_utf8_unchecked(
            core::slice::from_raw_parts(old_name, length)
        );
        length = 0;
        let mut ptr = new_name;
        while (*ptr) as char != '\0' {
            length += 1;
            ptr = ptr.offset(1);
        }
        let new_name = core::str::from_utf8_unchecked(
            core::slice::from_raw_parts(new_name, length)
        );
        println!("{} {}", old_name, new_name);
        ROOT_INODE.linkat(old_name, new_name)
    }
}

pub fn unlinkat(name: *const u8) -> isize {
    let mut length = 0;
    unsafe {
        let mut ptr = name;
        while *ptr != 0 {
            length += 1;
            ptr = ptr.offset(1);
        }
        let name = core::str::from_utf8_unchecked(
            core::slice::from_raw_parts(name, length)
        );
        ROOT_INODE.unlinkat(name)
    }
}