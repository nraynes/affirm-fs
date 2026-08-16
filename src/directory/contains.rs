use std::path::PathBuf;

use derive_new::new;

use crate::{AffirmFsError, Directory, File};

#[derive(new)]
pub struct Contains<'a> {
    value: &'a Directory,
}

impl<'a> Contains<'a> {
    /// Check if the directory contains a file and that file matches some other condition determined by a closure.
    fn has_file_and<F: Fn(&File) -> bool>(&self, path: &PathBuf, f: F) -> bool {
        if let Some(this_file) = self.value.files().get(path)
            && f(this_file)
        {
            return true;
        }
        false
    }

    /// Check if the directory contains a directory and that directory matches some other condition determined by a closure.
    fn has_dir_and<F: Fn(&Directory) -> bool>(&self, path: &PathBuf, f: F) -> bool {
        if let Some(this_dir) = self.value.directories().get(path)
            && f(this_dir)
        {
            return true;
        }
        false
    }

    /// Check if the directory contains a file and that file matches some other condition determined by a closure.
    /// Returns a Result in the case of operations that may not complete successfully.
    fn has_file_and_might<F: Fn(&File) -> Result<bool, AffirmFsError>>(
        &self,
        path: &PathBuf,
        f: F,
    ) -> Result<bool, AffirmFsError> {
        if let Some(this_file) = self.value.files().get(path)
            && f(this_file)?
        {
            return Ok(true);
        }
        Ok(false)
    }

    /// Check if the directory contains a directory and that directory matches some other condition determined by a closure.
    /// Returns a Result in the case of operations that may not complete successfully.
    fn has_dir_and_might<F: Fn(&Directory) -> Result<bool, AffirmFsError>>(
        &self,
        path: &PathBuf,
        f: F,
    ) -> Result<bool, AffirmFsError> {
        if let Some(this_dir) = self.value.directories().get(path)
            && f(this_dir)?
        {
            return Ok(true);
        }
        Ok(false)
    }

    /// Checks if this directory contains the structure of another directory. This essentially checks to see if the first
    /// layer of contents inside this directory at least contains the same contents as the provided directory, but may also
    /// contain additional contents.
    pub fn structure(&self, value: &Directory) -> bool {
        for (path, other_file) in value.files() {
            if !self.has_file_and(path, |f| f.eq().file(other_file)) {
                return false;
            }
        }

        for (path, other_dir) in value.directories() {
            if !self.has_dir_and(path, |d| d.eq().dir(other_dir)) {
                return false;
            }
        }
        true
    }

    /// The same as .structure, except it does a deep comparison on the contents of files for equality.
    pub fn deep_structure(&self, value: &Directory) -> Result<bool, AffirmFsError> {
        for (path, other_file) in value.files() {
            if !self.has_file_and_might(path, |f| f.deep_eq().file(other_file))? {
                return Ok(false);
            }
        }

        for (path, other_dir) in value.directories() {
            if !self.has_dir_and_might(path, |d| d.deep_eq().dir(other_dir))? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Checks to see if this directory contains a file matching the one provided.
    pub fn file(&self, value: &File) -> bool {
        self.has_file_and(value.path(), |f| f.eq().file(value))
    }

    /// Checks to see if this directory contains a file matching the one provided, and that the contents of the file
    /// matches the one provided.
    pub fn deep_file(&self, value: &File) -> Result<bool, AffirmFsError> {
        Ok(self.has_file_and_might(value.path(), |f| f.deep_eq().file(value))?)
    }

    /// Checks to see if this directory contains a file with the provided name.
    pub fn file_named(&self, value: &str) -> bool {
        self.value
            .files()
            .get(&self.value.path().join(value))
            .is_some()
    }

    /// Checks to see if this directory contains a directory matching the one provided.
    pub fn dir(&self, value: &Directory) -> bool {
        self.has_dir_and(value.path(), |d| d.eq().dir(value))
    }

    /// Checks to see if this directory contains a directory matching the one provided, and that the contents of the
    /// directory matches the one provided.
    pub fn deep_dir(&self, value: &Directory) -> Result<bool, AffirmFsError> {
        Ok(self.has_dir_and_might(value.path(), |d| d.deep_eq().dir(value))?)
    }

    /// Checks to see if this directory contains a directory with the provided name.
    pub fn dir_named(&self, value: &str) -> bool {
        self.value
            .directories()
            .get(&self.value.path().join(value))
            .is_some()
    }
}
