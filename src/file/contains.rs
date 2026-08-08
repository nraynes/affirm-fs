use std::fs;

use derive_new::new;

use crate::{AFile, contains_subslice};

#[derive(new)]
pub struct Contains<'a> {
    value: &'a AFile,
}

impl<'a> Contains<'a> {
    pub fn content<T>(&self, value: T) -> bool
    where
        T: AsRef<[u8]>,
    {
        if let Ok(file_contents) = fs::read(self.value.path())
            && contains_subslice(&file_contents, value.as_ref())
        {
            return true;
        }
        false
    }
}
