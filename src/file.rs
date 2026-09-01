mod contains;
mod deep_eq;
mod eq;
mod ne;

use contains::Contains;
use deep_eq::DeepEq;
use eq::Eq;
use ne::Ne;

use std::{
    fs,
    path::{Path, PathBuf},
};

use derive_new::new;

use crate::AffirmFsError;

#[derive(new, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct File {
    path: PathBuf,
    pub static_content: Option<Vec<u8>>,
}

impl From<&str> for File {
    fn from(value: &str) -> Self {
        Self::new(PathBuf::from(value), None)
    }
}

impl From<(&str, &str)> for File {
    fn from(value: (&str, &str)) -> Self {
        Self::new(PathBuf::from(value.0), Some(value.1.as_bytes().to_vec()))
    }
}

impl From<PathBuf> for File {
    fn from(value: PathBuf) -> Self {
        Self::new(value, None)
    }
}

impl From<(PathBuf, &str)> for File {
    fn from(value: (PathBuf, &str)) -> Self {
        Self::new(value.0, Some(value.1.as_bytes().to_vec()))
    }
}

/// Attempts to create Self that matches a file for a specific path on disk.
impl TryFrom<&Path> for File {
    type Error = AffirmFsError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        if value.is_file() && fs::exists(value)? {
            return Ok(Self::new(value.to_path_buf(), None));
        }
        Err(AffirmFsError::from(format!(
            "File not found at {:?}",
            value
        )))
    }
}

impl File {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Gets the contents of this file once and returns it as a byte vector.
    pub fn contents(&self) -> Result<Vec<u8>, AffirmFsError> {
        Ok(fs::read(&self.path)?)
    }

    /// Gets the contents of this file and caches it in memory for making matching decisions later on.
    /// If static_contents is already set, this will overwrite it.
    /// Use with caution for larger files!
    pub fn hold_contents_as_static(&mut self) -> Result<(), AffirmFsError> {
        self.static_content = Some(self.contents()?);
        Ok(())
    }

    pub fn take_and_hold_contents_as_static(mut self) -> Result<Self, AffirmFsError> {
        self.static_content = Some(self.contents()?);
        Ok(self)
    }

    pub fn hash(&self) -> Result<String, AffirmFsError> {
        Ok(sha256::digest(fs::read(&self.path)?))
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
}
