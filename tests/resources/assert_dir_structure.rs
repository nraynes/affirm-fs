use std::{collections::HashMap, path::PathBuf};

use affirm_fs::Directory;
use mocked_up::file_system::TempDir;

pub fn assert_dir_structure<const N: usize>(
    temp: &TempDir,
    test_dir: &Directory,
    contents_map: &[(&PathBuf, &str); N],
) {
    let contents_hashmap: HashMap<&PathBuf, &str> = HashMap::from_iter(*contents_map);

    // Test files.
    for file in temp.files().values() {
        let file_model = test_dir.file(file.path()).unwrap();
        assert_eq!(file_model.path(), file.path());
        let expected_contents = contents_hashmap.get(file_model.path()).unwrap();
        assert!(file_model.deep_eq().content(expected_contents).unwrap());
    }

    // Test directories.
    for directory in temp.dirs().values() {
        let dir_model = test_dir.dir(directory.path()).unwrap();
        assert_eq!(dir_model.path(), directory.path());
        assert_dir_structure(directory, dir_model, contents_map);
    }
}
