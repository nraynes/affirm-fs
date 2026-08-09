use std::path::PathBuf;

use derive_new::new;

use crate::{AffirmFsError, LazyLoad};

#[derive(new, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct ASymLink {
    path: PathBuf,
    links_to: LazyLoad<PathBuf>,
}

impl From<&str> for ASymLink {
    fn from(value: &str) -> Self {
        Self::new(PathBuf::from(value), LazyLoad::Stale)
    }
}

impl TryFrom<PathBuf> for ASymLink {
    type Error = AffirmFsError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if value.is_symlink() {
            return Ok(Self {
                path: value,
                links_to: LazyLoad::Stale,
            });
        }
        Err(AffirmFsError::from(format!(
            "SymLink not found at {:?}",
            value
        )))
    }
}

impl ASymLink {
    fn retrieve_link(&mut self) {
        match self.path.read_link() {
            Ok(link) => {
                self.links_to = LazyLoad::Has(link);
            }
            Err(e) => {
                self.links_to = LazyLoad::Err(AffirmFsError::from(e));
            }
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn links_to(&mut self) -> Result<&PathBuf, AffirmFsError> {
        if self.links_to == LazyLoad::Stale {
            self.retrieve_link();
        }
        match &self.links_to {
            LazyLoad::Stale => Err(AffirmFsError::from("Could not retrieve link.")),
            LazyLoad::Has(link) => Ok(link),
            LazyLoad::Err(e) => Err(e.clone()),
        }
    }
}
