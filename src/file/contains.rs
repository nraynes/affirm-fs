use derive_new::new;

use crate::{AffirmFsError, File, contains_subslice};

#[derive(new)]
pub struct Contains<'a> {
    value: &'a File,
}

impl<'a> Contains<'a> {
    /// Checks for whether the provided byte slice is present in the files contents.
    pub fn content<T>(&self, value: T) -> Result<bool, AffirmFsError>
    where
        T: AsRef<[u8]>,
    {
        let file_contents = self.value.contents()?;
        Ok(contains_subslice(&file_contents, value.as_ref()))
    }
}
