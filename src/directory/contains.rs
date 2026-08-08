use derive_new::new;

use crate::{ADirectory, AFile, ASymLink, compare_opt};

#[derive(new)]
pub struct Contains<'a> {
    value: &'a ADirectory,
}

impl<'a> Contains<'a> {
    pub fn structure(&self, value: &ADirectory) -> bool {
        for (path, other_file) in value.files() {
            if compare_opt(self.value.files().get(path), other_file) {
                return false;
            }
        }
        for (path, other_dir) in value.directories() {
            if compare_opt(self.value.directories().get(path), other_dir) {
                return false;
            }
        }
        for (path, other_link) in value.links() {
            if compare_opt(self.value.links().get(path), other_link) {
                return false;
            }
        }
        true
    }

    pub fn file(&self, value: &AFile) -> bool {
        if let Some(this_file) = self.value.files().get(value.path())
            && this_file == value
        {
            return true;
        }
        false
    }

    pub fn link(&self, value: &ASymLink) -> bool {
        if let Some(this_link) = self.value.links().get(value.path())
            && this_link == value
        {
            return true;
        }
        false
    }

    pub fn dir(&self, value: &ADirectory) -> bool {
        if let Some(this_dir) = self.value.directories().get(value.path())
            && this_dir == value
        {
            return true;
        }
        false
    }
}
