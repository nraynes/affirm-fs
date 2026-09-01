use derive_new::new;

use crate::{AffirmFsError, Directory};

#[derive(new)]
pub struct Eq<'a> {
    value: &'a Directory,
}

impl<'a> Eq<'a> {
    fn check_contents_count(&self, value: &Directory) -> bool {
        self.value.files().len() == value.files().len()
            && self.value.directories().len() == value.directories().len()
    }

    /// Checks whether a directory matches another directory, but does not check file contents.
    pub fn dir(&self, value: &Directory) -> bool {
        // Match paths.
        if self.value.path() != value.path() || !self.check_contents_count(value) {
            return false;
        }

        // Match files.
        for (file_path, this_file) in self.value.files() {
            if let Some(other_file) = value.files().get(file_path) {
                if !this_file.eq().file(other_file) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Match directories.
        for (dir_path, this_dir) in self.value.directories() {
            if let Some(other_dir) = value.directories().get(dir_path) {
                if !this_dir.eq().dir(other_dir) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Checks whether a directory matches another directory, but does not check file contents. Uses last component
    /// of path for comparison.
    pub fn dir_weak(&self, value: &Directory) -> Result<bool, AffirmFsError> {
        if !self.check_contents_count(value) {
            return Ok(false);
        }

        // Match files.
        for (file_path, this_file) in self.value.files() {
            let value_path = value.path().join(
                file_path
                    .file_name()
                    .ok_or("Could not extract file name.")?,
            );
            if let Some(other_file) = value.files().get(&value_path) {
                if !this_file.eq().file_name(other_file)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        // Match directories.
        for (dir_path, this_dir) in self.value.directories() {
            let value_path = &value
                .path()
                .join(dir_path.file_name().ok_or("Could not extract file name.")?);
            if let Some(other_dir) = value.directories().get(value_path) {
                if !this_dir.eq().dir_weak(other_dir)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
