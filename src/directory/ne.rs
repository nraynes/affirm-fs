use derive_new::new;

use crate::{Directory, directory::eq::Eq};

#[derive(new)]
pub struct Ne<'a> {
    value: &'a Directory,
}

impl<'a> Ne<'a> {
    /// Checks whether this directory does not match another directory.
    pub fn dir(&self, value: &Directory) -> bool {
        !Eq::new(self.value).dir(value)
    }
}
