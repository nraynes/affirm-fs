mod contains;
use contains::Contains;

use derive_getters::Getters;

use std::{
    collections::HashMap,
    fs::{self, canonicalize},
    path::{Path, PathBuf},
};

use derive_new::new;

use crate::{AFile, ASymLink, AffirmFsError};

#[derive(Getters, new, PartialEq, Eq, Default, Clone)]
pub struct ADirectory {
    path: PathBuf,
    files: HashMap<PathBuf, AFile>,
    directories: HashMap<PathBuf, Self>,
    links: HashMap<PathBuf, ASymLink>,
}

impl TryFrom<&Path> for ADirectory {
    type Error = AffirmFsError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let mut files = HashMap::new();
        let mut directories = HashMap::new();
        let mut links = HashMap::new();

        for dir_entry in fs::read_dir(&value)? {
            let path = dir_entry?.path();

            if path.is_dir() {
                directories.insert(canonicalize(&path)?, Self::try_from(path)?);
            } else if path.is_file() {
                files.insert(path.clone(), AFile::try_from(path)?);
            } else if path.is_symlink() {
                links.insert(path.clone(), ASymLink::try_from(path)?);
            }
        }

        Ok(Self::new(value.to_path_buf(), files, directories, links))
    }
}

impl TryFrom<&PathBuf> for ADirectory {
    type Error = AffirmFsError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl TryFrom<PathBuf> for ADirectory {
    type Error = AffirmFsError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl ADirectory {
    pub fn empty(path: PathBuf) -> Self {
        Self {
            path,
            files: HashMap::new(),
            directories: HashMap::new(),
            links: HashMap::new(),
        }
    }

    pub fn insert_dir(&mut self, value: Self) {
        self.directories.insert(value.path().clone(), value);
    }

    pub fn insert_file(&mut self, value: AFile) {
        self.files.insert(value.path().clone(), value);
    }

    pub fn insert_link(&mut self, value: ASymLink) {
        self.links.insert(value.path().clone(), value);
    }

    pub fn contains<'a>(&'a self) -> Contains<'a> {
        Contains::new(self)
    }
}
