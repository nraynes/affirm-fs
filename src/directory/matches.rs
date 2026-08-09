use derive_new::new;

use crate::{ADirectory, AffirmFsError};

#[derive(new)]
pub struct Matches<'a> {
    value: &'a mut ADirectory,
}

impl<'a> Matches<'a> {
    pub fn dir(&mut self, value: &mut ADirectory) -> Result<bool, AffirmFsError> {
        if let Some(this_dir) = self.value.directories_mut().get_mut(value.path())
            && this_dir == value
        {
            for (file_path, this_file) in this_dir.files_mut() {
                let other_file = value
                    .files_mut()
                    .get_mut(file_path)
                    .ok_or("A problem was encountered while attempting to get file contents.")?;
                if !this_file.matches().file(other_file)? {
                    return Ok(false);
                }
            }
            return Ok(true);
        }
        Ok(false)
    }
}
