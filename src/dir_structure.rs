use crate::{ADirectory, AFile, ASymLink};

#[derive(Default)]
pub struct DirStructure {
    root: ADirectory,
}

impl DirStructure {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dir<F>(mut self, f: F) -> Self
    where
        F: Fn(Self) -> ADirectory,
    {
        self.root.insert_dir(f(Self::new()));
        self
    }

    pub fn file(mut self, value: AFile) -> Self {
        self.root.insert_file(value);
        self
    }

    pub fn link(mut self, value: ASymLink) -> Self {
        self.root.insert_link(value);
        self
    }

    pub fn build(self) -> ADirectory {
        self.root.clone()
    }
}
