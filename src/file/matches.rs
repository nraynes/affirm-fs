use derive_new::new;

use crate::{AFile, AffirmFsError};

#[derive(new)]
pub struct Matches<'a> {
    value: &'a mut AFile,
}

impl<'a> Matches<'a> {
    pub fn file(&mut self, value: &mut AFile) -> Result<bool, AffirmFsError> {
        Ok(self.value.hash()? == value.hash()?)
    }

    pub fn content<T>(&mut self, value: T) -> Result<bool, AffirmFsError>
    where
        T: AsRef<[u8]>,
    {
        Ok(self.value.hash()? == sha256::digest(value.as_ref()))
    }
}
