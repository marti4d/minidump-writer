#![allow(unused)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("File or directory already exists")]
    AlreadyExists,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct FakeFilesystem {
    root: Directory,
}

impl FakeFilesystem {
    pub fn root(&self) -> &Directory {
        &self.root
    }
    pub fn copy<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
        &self,
        from: P,
        to: Q,
    ) -> std::io::Result<u64> {
        todo!()
    }
    pub fn metadata<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> std::io::Result<Metadata> {
        todo!()
    }
    pub fn read<P: AsRef<std::path::Path>>(&self,path: P) -> std::io::Result<Vec<u8>> {
        todo!()
    }
    pub fn read_dir<P: AsRef<std::path::Path>>(&self,path: P) -> std::io::Result<ReadDir> {
        todo!()
    }
    pub fn read_link<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> std::io::Result<std::path::PathBuf> {
        todo!()
    }
    pub fn read_to_string<P: AsRef<std::path::Path>>(&self,path: P) -> std::io::Result<String> {
        todo!()
    }
    pub fn remove_file<P: AsRef<std::path::Path>>(&self,path: P) -> std::io::Result<()> {
        todo!()
    }
    pub fn create_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<File> {
        todo!();
    }
    pub fn open_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<File> {
        todo!();
    }
    pub fn is_dir(&self, path: &std::path::Path) -> bool {
        todo!();
    }
    pub fn exists(&self, path: &std::path::Path) -> bool {
        todo!();
    }
}

impl Default for FakeFilesystem {
    fn default() -> Self {
        Self {
            root: Directory::default(),
        }
    }
}

#[derive(Debug)]
pub struct File {}

impl File {
    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!();
    }
    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!();
    }
    pub fn flush(&mut self) -> std::io::Result<()> {
        todo!();
    }
    pub fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        todo!();
    }
    pub fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        todo!();
    }
    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<usize> {
        todo!();
    }
    pub fn write_at(&self, buf: &[u8], offset: u64) -> std::io::Result<usize> {
        todo!();
    }
    pub fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<()> {
        todo!();
    }
    pub fn write_all_at(&self, buf: &[u8], offset: u64) -> std::io::Result<()> {
        todo!();
    }
}

#[derive(Debug)]
pub struct ReadDir {}

impl ReadDir {
    pub fn next(&mut self) -> Option<std::io::Result<DirEntry>>{
        todo!();
    }
}

#[derive(Debug)]
pub struct DirEntry {}

impl DirEntry {
    pub fn file_name(&self) -> std::ffi::OsString {
        todo!()
    }
    pub fn path(&self) -> std::path::PathBuf {
        todo!();
    }
}

#[derive(Debug, Default)]
pub struct Metadata {
    len: u64,
}

impl Metadata {
    pub fn len(&self) -> u64 {
        self.len
    }
}

#[derive(Debug)]
enum FileRecord {
    Directory(Directory),
    Regular(RegularFile),
    Symlink(Symlink),
}

type FileRecordLink = Arc<FileRecord>;

#[derive(Debug, Default)]
pub struct Directory {
    metadata: Metadata,
    files: Mutex<HashMap<String, FileRecordLink>>,
}

impl Directory {
    fn into_link(self) -> FileRecordLink {
        Arc::new(FileRecord::Directory(self))
    }
    pub fn add_directory<N: Into<String>>(&self, name: N) -> Result<()> {
        let mut files = self.files.lock().unwrap();
        let std::collections::hash_map::Entry::Vacant(entry) = files.entry(name.into()) else {
            return Err(Error::AlreadyExists);
        };
        entry.insert(Directory::default().into_link());
        Ok(())
    }
    pub fn add_file<N: Into<String>>(&self, name: N) -> Result<()> {
        let mut files = self.files.lock().unwrap();
        let std::collections::hash_map::Entry::Vacant(entry) = files.entry(name.into()) else {
            return Err(Error::AlreadyExists);
        };
        entry.insert(RegularFile::default().into_link());
        Ok(())
    }
    pub fn add_symlink<N: Into<String>, T: Into<String>>(&self, name: N, target: T) -> Result<()> {
        let mut files = self.files.lock().unwrap();
        let std::collections::hash_map::Entry::Vacant(entry) = files.entry(name.into()) else {
            return Err(Error::AlreadyExists);
        };
        entry.insert(Symlink::with_target(target.into()).into_link());
        Ok(())
    }
}

#[derive(Debug, Default)]
struct RegularFile {
    metadata: Metadata,
    bytes: Vec<u8>,
}

impl RegularFile {
    fn into_link(self) -> FileRecordLink {
        Arc::new(FileRecord::Regular(self))
    }
}

#[derive(Debug)]
struct Symlink {
    metadata: Metadata,
    target: String,
}

impl Symlink {
    fn with_target(target: String) -> Self {
        Self {
            metadata: Metadata::default(),
            target,
        }
    }
    fn into_link(self) -> FileRecordLink {
        Arc::new(FileRecord::Symlink(self))
    }
}

