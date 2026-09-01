use affirm_fs::{Directory, File};

use crate::resources::init_temp_env;

#[test]
fn file_on_disk_does_equal() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let inner_file = test_dir.file(temp.env().path().join("f1")).unwrap();

    assert!(
        inner_file
            .deep_eq()
            .file(&File::try_from(temp.env().path().join("f1")).unwrap())
            .unwrap()
    );
}

#[test]
fn file_static_does_equal() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let inner_file = test_dir.file(temp.env().path().join("f1")).unwrap();

    assert!(
        inner_file
            .deep_eq()
            .file(&File::from((
                temp.env().path().join("f1"),
                "Test content for f1."
            )))
            .unwrap()
    );
}

#[test]
fn file_on_disk_does_not_equal() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let inner_file = test_dir.file(temp.env().path().join("f1")).unwrap();

    assert!(
        !inner_file
            .deep_eq()
            .file(&File::try_from(temp.env().path().join("f2")).unwrap())
            .unwrap()
    );
}

#[test]
fn file_static_does_not_equal_content() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let inner_file = test_dir.file(temp.env().path().join("f1")).unwrap();

    assert!(
        !inner_file
            .deep_eq()
            .file(&File::from((
                temp.env().path().join("f1"),
                "Test content for f2."
            )))
            .unwrap()
    );
}

#[test]
fn file_static_does_not_equal_path() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let inner_file = test_dir.file(temp.env().path().join("f1")).unwrap();

    assert!(
        !inner_file
            .deep_eq()
            .file(&File::from((
                temp.env().path().join("f2"),
                "Test content for f1."
            )))
            .unwrap()
    );
}
