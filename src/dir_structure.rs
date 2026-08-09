use std::path::Path;

use crate::{ADirectory, AFile, ASymLink};

pub struct DirStructure {
    root: ADirectory,
}

impl DirStructure {
    pub fn new(path: &Path) -> Self {
        Self {
            root: ADirectory::empty(path.to_path_buf()),
        }
    }

    pub fn dir<F>(mut self, path: &Path, f: F) -> Self
    where
        F: Fn(Self) -> ADirectory,
    {
        self.root.insert_dir(f(Self::new(path)));
        self
    }

    pub fn file(mut self, value: &str) -> Self {
        self.root.insert_file(AFile::from(value));
        self
    }

    pub fn link(mut self, value: &str) -> Self {
        self.root.insert_link(ASymLink::from(value));
        self
    }

    pub fn build(self) -> ADirectory {
        self.root.clone()
    }
}
