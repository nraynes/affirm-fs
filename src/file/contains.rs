use derive_new::new;

use crate::{AFile, AffirmFsError, contains_subslice};

#[derive(new)]
pub struct Contains<'a> {
    value: &'a mut AFile,
}

impl<'a> Contains<'a> {
    pub fn content<T>(&mut self, value: T) -> Result<bool, AffirmFsError>
    where
        T: AsRef<[u8]>,
    {
        let file_contents = self.value.contents()?;
        Ok(contains_subslice(&file_contents, value.as_ref()))
    }
}
