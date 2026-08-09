mod contains;
mod contents;

use contains::Contains;
use contents::Contents;

use std::{fs, path::PathBuf};

use derive_new::new;

use crate::AffirmFsError;

#[derive(new, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AFile {
    path: PathBuf,
    contents: Contents,
}

impl From<&str> for AFile {
    fn from(value: &str) -> Self {
        Self::new(PathBuf::from(value), Contents::Stale)
    }
}

impl TryFrom<PathBuf> for AFile {
    type Error = AffirmFsError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if value.is_file() {
            return Ok(Self::new(value, Contents::Stale));
        }
        Err(AffirmFsError::from(format!(
            "File not found at {:?}",
            value
        )))
    }
}

impl AFile {
    fn update_contents(&mut self) {
        match fs::read(&self.path) {
            Ok(file_contents) => {
                self.contents = Contents::Has(file_contents);
            }
            Err(e) => {
                self.contents = Contents::Err(AffirmFsError::from(e));
            }
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn contents(&mut self) -> Result<&[u8], AffirmFsError> {
        if self.contents == Contents::Stale {
            self.update_contents();
        }
        match &self.contents {
            Contents::Stale => Err(AffirmFsError::from("Could not retrieve file contents.")),
            Contents::Has(file_contents) => Ok(file_contents),
            Contents::Err(e) => Err(e.clone()),
        }
    }

    pub fn contains<'a>(&'a mut self) -> Contains<'a> {
        Contains::new(self)
    }
}
