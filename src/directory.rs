mod contains;
mod matches;

use contains::Contains;
use matches::Matches;

use derive_getters::Getters;

use std::{
    collections::HashMap,
    fs::{self, canonicalize},
    path::{Path, PathBuf},
};

use derive_new::new;

use crate::{AFile, ASymLink, AffirmFsError};

#[derive(Getters, new, PartialEq, Eq, Default, Clone, Debug)]
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

    pub fn directories_mut(&mut self) -> &mut HashMap<PathBuf, Self> {
        &mut self.directories
    }

    pub fn files_mut(&mut self) -> &mut HashMap<PathBuf, AFile> {
        &mut self.files
    }

    pub fn links_mut(&mut self) -> &mut HashMap<PathBuf, ASymLink> {
        &mut self.links
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

    pub fn matches<'a>(&'a mut self) -> Matches<'a> {
        Matches::new(self)
    }

    pub fn dir(&self, path: &Path) -> Option<&Self> {
        let parsed_path = if path.is_relative() {
            path.to_path_buf()
        } else {
            path.canonicalize()
                .ok()?
                .strip_prefix(&self.path)
                .ok()?
                .to_path_buf()
        };
        let mut path_iter = parsed_path.into_iter();
        if let Some(next) = path_iter.next() {
            let path_to_next = self.path.join(next);
            if let Some(next_dir) = self.directories().get(&path_to_next) {
                return next_dir.dir(path_iter.as_path());
            }
        } else {
            return Some(self);
        }
        None
    }

    pub fn file(&self, path: &Path) -> Option<&AFile> {
        if let Some(parent_path) = path.parent()
            && let Some(parent_dir) = self.dir(parent_path)
        {
            return parent_dir
                .files()
                .get(&parent_dir.path().join(path.file_name()?));
        }
        None
    }

    pub fn link(&self, path: &Path) -> Option<&ASymLink> {
        if let Some(parent_path) = path.parent()
            && let Some(parent_dir) = self.dir(parent_path)
        {
            return parent_dir
                .links()
                .get(&parent_dir.path().join(path.file_name()?));
        }
        None
    }
}
