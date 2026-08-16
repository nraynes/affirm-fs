use derive_new::new;

use crate::{AffirmFsError, File};

#[derive(new)]
pub struct Eq<'a> {
    value: &'a File,
}

impl<'a> Eq<'a> {
    /// Checks if this files path matches that of the provided file.
    pub fn file(&self, value: &File) -> bool {
        self.value.path() == value.path()
    }

    /// Checks whether the contents of this file is equal to some content by hashing the contents and comparing the hashes.
    pub fn content<T>(&self, value: T) -> Result<bool, AffirmFsError>
    where
        T: AsRef<[u8]>,
    {
        Ok(self.value.hash()? == sha256::digest(value.as_ref()))
    }
}
