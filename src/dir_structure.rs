use std::path::{Path, PathBuf};

use crate::{Directory, File};

#[derive(Debug)]
pub struct DirStructure {
    root: Directory,
}

impl DirStructure {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            root: Directory::empty(PathBuf::from(path.as_ref())),
        }
    }

    pub fn dir<F>(mut self, name: &str, f: F) -> Self
    where
        F: Fn(Self) -> Directory,
    {
        let path = self.root.path().join(name);
        self.root = self.root.insert_dir(f(Self::new(path)));
        self
    }

    pub fn file(mut self, name: &str) -> Self {
        let path = self.root.path().join(name);
        self.root = self.root.insert_file(File::empty(path));
        self
    }

    pub fn file_with_contents(mut self, name: &str, contents: &str) -> Self {
        let path = self.root.path().join(name);
        self.root = self.root.insert_file(File::from((path, contents)));
        self
    }

    pub fn build(self) -> Directory {
        self.root
    }
}
