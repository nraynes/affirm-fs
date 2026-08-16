use derive_new::new;

use crate::{AffirmFsError, File};

#[derive(new)]
pub struct Ne<'a> {
    value: &'a File,
}

impl<'a> Ne<'a> {
    /// Checks if this files path does not match that of the provided file.
    pub fn file(&self, value: &File) -> bool {
        self.value.path() != value.path()
    }

    /// Checks whether the contents of this file is not equal to some content by hashing the contents and
    /// comparing the hashes.
    pub fn content<T>(&self, value: T) -> Result<bool, AffirmFsError>
    where
        T: AsRef<[u8]>,
    {
        Ok(self.value.hash()? != sha256::digest(value.as_ref()))
    }
}
