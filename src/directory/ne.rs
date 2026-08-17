use derive_new::new;

use crate::{AffirmFsError, Directory, directory::eq::Eq};

#[derive(new)]
pub struct Ne<'a> {
    value: &'a Directory,
}

impl<'a> Ne<'a> {
    /// Checks whether this directory does not match another directory.
    pub fn dir(&self, value: &Directory) -> bool {
        !Eq::new(self.value).dir(value)
    }

    /// Checks whether this directory does not match another directory. Uses final component of path for comparisons.
    pub fn dir_weak(&self, value: &Directory) -> Result<bool, AffirmFsError> {
        Ok(!Eq::new(self.value).dir_weak(value)?)
    }
}
