mod contains;
mod deep_eq;
mod eq;
mod ne;

use contains::Contains;
use deep_eq::DeepEq;
use eq::Eq;
use ne::Ne;

use derive_getters::Getters;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use derive_new::new;

use crate::{AffirmFsError, File};

#[derive(Getters, new, PartialEq, Eq, Default, Clone, Debug)]
pub struct Directory {
    path: PathBuf,
    files: HashMap<PathBuf, File>,
    directories: HashMap<PathBuf, Self>,
}

/// Attempts to create Self that matches the directory and file structure of a specific path on disk.
impl TryFrom<&Path> for Directory {
    type Error = AffirmFsError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let mut files = HashMap::new();
        let mut directories = HashMap::new();

        for dir_entry in fs::read_dir(&value)? {
            let path = dir_entry?.path();

            if path.is_dir() {
                directories.insert(path.canonicalize()?, Self::try_from(path)?);
            } else if path.is_file() {
                files.insert(path.clone(), File::try_from(path.as_path())?);
            }
        }

        Ok(Self::new(value.to_path_buf(), files, directories))
    }
}

impl TryFrom<&PathBuf> for Directory {
    type Error = AffirmFsError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl TryFrom<PathBuf> for Directory {
    type Error = AffirmFsError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl Directory {
    pub fn empty(path: PathBuf) -> Self {
        Self {
            path,
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    /// Will acquire the contents of all files in this directory and all subdirectories for making matching decisions
    /// later on. Use with Extreme Caution! Larger files can cause crashes!
    pub fn acquire_contents(&mut self) -> Result<(), AffirmFsError> {
        // Acquire file contents in this directory.
        for file in self.files.values_mut() {
            file.hold_contents_as_static()?;
        }

        // Acquire file contents for all files in subdirectories.
        for directory in self.directories.values_mut() {
            directory.acquire_contents()?;
        }

        Ok(())
    }

    /// Takes ownership of directory to acquire contents then returns ownership to maintain immutability after modification.
    pub fn take_and_acquire_contents(mut self) -> Result<Self, AffirmFsError> {
        self.acquire_contents()?;
        Ok(self)
    }

    pub fn insert_dir(mut self, value: Self) -> Self {
        self.directories.insert(value.path().clone(), value);
        self
    }

    pub fn insert_file(mut self, value: File) -> Self {
        self.files.insert(value.path().clone(), value);
        self
    }

    pub fn contains<'a>(&'a self) -> Contains<'a> {
        Contains::new(self)
    }

    pub fn deep_eq<'a>(&'a self) -> DeepEq<'a> {
        DeepEq::new(self)
    }

    pub fn eq<'a>(&'a self) -> Eq<'a> {
        Eq::new(self)
    }

    pub fn ne<'a>(&'a self) -> Ne<'a> {
        Ne::new(self)
    }

    /// Gets the directory at the given path, if it exists.
    /// The given path can be either an absolute path with the prefix to the absolute path matching this directory's path,
    /// or a relative path that is relative to this directory's path.
    /// The directory structure will be traversed to retrieve a directory that is multiple subdirectories into the structure.
    /// If the directory cannot be found or an invalid path is given, None is returned.
    pub fn dir<P: AsRef<Path>>(&self, path: P) -> Option<&Self> {
        let path = path.as_ref();
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

    /// Gets the file at the given path, if it exists.
    /// The given path can be either an absolute path with the prefix to the absolute path matching this directory's path,
    /// or a relative path that is relative to this directory's path.
    /// The directory structure will be traversed to retrieve a file that is multiple subdirectories into the structure.
    /// If the file cannot be found or an invalid path is given, None is returned.
    pub fn file<P: AsRef<Path>>(&self, path: P) -> Option<&File> {
        let path = path.as_ref();
        if let Some(parent_path) = path.parent()
            && let Some(parent_dir) = self.dir(parent_path)
        {
            return parent_dir
                .files()
                .get(&parent_dir.path().join(path.file_name()?));
        }
        None
    }
}
