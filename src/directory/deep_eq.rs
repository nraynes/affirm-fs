use std::path::PathBuf;

use derive_new::new;

use crate::{AffirmFsError, Directory, File};

#[derive(new)]
pub struct DeepEq<'a> {
    value: &'a Directory,
}

impl<'a> DeepEq<'a> {
    fn check_contents_count(&self, value: &Directory) -> bool {
        self.value.files().len() == value.files().len()
            && self.value.directories().len() == value.directories().len()
    }

    fn match_files_and<
        F: Fn(&File, &File) -> Result<bool, AffirmFsError>,
        G: Fn(&PathBuf) -> Result<PathBuf, AffirmFsError>,
    >(
        &self,
        value: &Directory,
        f_path_condition: F,
        f_file_path: G,
    ) -> Result<bool, AffirmFsError> {
        for (file_path, this_file) in self.value.files() {
            if let Some(other_file) = value.files().get(&f_file_path(file_path)?) {
                if !f_path_condition(this_file, other_file)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn match_dirs_and<
        F: Fn(&Directory, &Directory) -> Result<bool, AffirmFsError>,
        G: Fn(&PathBuf) -> Result<PathBuf, AffirmFsError>,
    >(
        &self,
        value: &Directory,
        f_path_condition: F,
        f_dir_path: G,
    ) -> Result<bool, AffirmFsError> {
        for (dir_path, this_dir) in self.value.directories() {
            if let Some(other_dir) = value.directories().get(&f_dir_path(dir_path)?) {
                if !f_path_condition(this_dir, other_dir)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Checks that this directory is exactly equal to another directory, and that all files in this directory
    /// and all subdirectories contents matches the contents of the respective files in the other directory.
    pub fn dir(&self, value: &Directory) -> Result<bool, AffirmFsError> {
        if self.value.path() != value.path()
            || !self.check_contents_count(value)
            || !self.match_files_and(
                value,
                |tf, of| tf.deep_eq().file(of),
                |fp| Ok(fp.to_path_buf()),
            )?
            || !self.match_dirs_and(
                value,
                |td, od| td.deep_eq().dir(od),
                |dp| Ok(dp.to_path_buf()),
            )?
        {
            return Ok(false);
        }
        Ok(true)
    }

    /// Checks that all files in this directory and all subdirectories contents matches the contents of the
    /// respective files in the other directory. Only the final component of the path is compared.
    pub fn dir_weak(&self, value: &Directory) -> Result<bool, AffirmFsError> {
        if !self.check_contents_count(value)
            || !self.match_files_and(
                value,
                |tf, of| tf.deep_eq().file_contents(of),
                |fp| {
                    Ok(value
                        .path()
                        .join(fp.file_name().ok_or("Could not extract file name.")?))
                },
            )?
            || !self.match_dirs_and(
                value,
                |td, od| td.deep_eq().dir_weak(od),
                |dp| {
                    Ok(value
                        .path()
                        .join(dp.file_name().ok_or("Could not extract directory name.")?))
                },
            )?
        {
            return Ok(false);
        }
        Ok(true)
    }
}
