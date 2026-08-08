use std::path::PathBuf;

use derive_getters::Getters;
use derive_new::new;

use crate::AffirmFsError;

#[derive(Getters, new, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ASymLink {
    path: PathBuf,
}

impl TryFrom<PathBuf> for ASymLink {
    type Error = AffirmFsError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if value.is_symlink() {
            return Ok(Self { path: value });
        }
        Err(AffirmFsError::from(format!(
            "SymLink not found at {:?}",
            value
        )))
    }
}
