use std::path::PathBuf;

use affirm_fs::{Directory, File};

use crate::resources::init_temp_env;

#[test]
fn with_path_end_true() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let fake_path = PathBuf::from("/test/direct/something");

    let inner_file = test_dir.file(temp.env().path().join("f1")).unwrap();

    assert!(
        inner_file
            .eq()
            .file_name(&File::try_from(fake_path.join("f1")).unwrap())
            .unwrap()
    );
}

#[test]
fn with_path_end_false() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let fake_path = PathBuf::from("/test/direct/something");

    let inner_file = test_dir.file(temp.env().path().join("f1")).unwrap();

    assert!(
        !inner_file
            .eq()
            .file_name(&File::try_from(fake_path.join("f2")).unwrap())
            .unwrap()
    );
}
