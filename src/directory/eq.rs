use derive_new::new;

use crate::Directory;

#[derive(new)]
pub struct Eq<'a> {
    value: &'a Directory,
}

impl<'a> Eq<'a> {
    /// Checks whether a directory matches another directory, but does not check file contents.
    pub fn dir(&self, value: &Directory) -> bool {
        // Match paths.
        if self.value.path() != value.path() {
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
}
