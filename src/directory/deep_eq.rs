use derive_new::new;

use crate::{AffirmFsError, Directory};

#[derive(new)]
pub struct DeepEq<'a> {
    value: &'a Directory,
}

impl<'a> DeepEq<'a> {
    /// Checks that this directory is exactly equal to another directory, and that all files in this directory
    /// and all subdirectories contents matches the contents of the respective files in the other directory.
    pub fn dir(&self, value: &Directory) -> Result<bool, AffirmFsError> {
        // Match paths.
        if self.value.path() != value.path() {
            return Ok(false);
        }

        // Check file count.
        if self.value.files().len() != value.files().len() {
            return Ok(false);
        }

        // Check directory count.
        if self.value.directories().len() != value.directories().len() {
            return Ok(false);
        }

        // Match files.
        for (file_path, this_file) in self.value.files() {
            if let Some(other_file) = value.files().get(file_path) {
                if !this_file.deep_eq().file(other_file)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        // Match directories.
        for (dir_path, this_dir) in self.value.directories() {
            if let Some(other_dir) = value.directories().get(dir_path) {
                if !this_dir.deep_eq().dir(other_dir)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
