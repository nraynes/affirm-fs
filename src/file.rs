mod contains;
use contains::Contains;

use std::path::PathBuf;

use derive_getters::Getters;
use derive_new::new;

use crate::AffirmFsError;

#[derive(Getters, new, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AFile {
    path: PathBuf,
}

impl TryFrom<PathBuf> for AFile {
    type Error = AffirmFsError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if value.is_file() {
            return Ok(Self { path: value });
        }
        Err(AffirmFsError::from(format!(
            "File not found at {:?}",
            value
        )))
    }
}

impl AFile {
    pub fn contains<'a>(&'a self) -> Contains<'a> {
        Contains::new(self)
    }
}
