use derive_new::new;

use crate::{AffirmFsError, File};

#[derive(new)]
pub struct DeepEq<'a> {
    value: &'a File,
}

impl<'a> DeepEq<'a> {
    fn file_and<F: Fn(&File, &File) -> Result<bool, AffirmFsError>>(
        &self,
        value: &File,
        f: F,
    ) -> Result<bool, AffirmFsError> {
        // If user defined static content is present, match with that.
        if let Some(static_content) = &value.static_content {
            if !f(self.value, value)? {
                return Ok(false);
            }
            return Ok(self.content(static_content)?);
        }

        // If no static content is provided, attempt to hash contents of file on disk.
        Ok(self.value.hash()? == value.hash()?)
    }

    /// Checks whether this files contents are equal to another files contents by hashing the contents and comparing
    /// the hashes. By default, if user-provided static content is set on the provided argument file, that will be
    /// compared to the contents of the file on disk, and the paths will be compared for equality.
    /// If no static content is set, then this files contents will be compared with the contents of the provided argument
    /// files contents on disk, and their file paths will not be compared.
    pub fn file(&self, value: &File) -> Result<bool, AffirmFsError> {
        self.file_and(value, |tf: &File, of: &File| Ok(tf.path() == of.path()))
    }

    // Compares this files contents with another files contents. Only the final component of the path is compared.
    pub fn file_contents(&self, value: &File) -> Result<bool, AffirmFsError> {
        self.file_and(value, |tf: &File, of: &File| {
            Ok(tf
                .path()
                .file_name()
                .ok_or("Could not retrieve file name of path to self.")?
                == of
                    .path()
                    .file_name()
                    .ok_or("Could not retrieve the file name of given directory.")?)
        })
    }

    /// Checks whether the contents of this file is equal to some content by hashing the contents and comparing the hashes.
    pub fn content<T>(&self, value: T) -> Result<bool, AffirmFsError>
    where
        T: AsRef<[u8]>,
    {
        Ok(self.value.hash()? == sha256::digest(value.as_ref()))
    }
}
