use affirm_fs::{Directory, File};

use crate::resources::init_temp_env;

#[test]
fn does_contain() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let file = File::from((temp.env().path().join("f1"), "Test content for f1."));

    assert!(test_dir.contains().file(&file));
}

#[test]
fn does_contain_deep() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let file = File::from((temp.env().path().join("f1"), "Test content for f1."));

    assert!(test_dir.contains().deep_file(&file).unwrap());
}

#[test]
fn contains_named() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    assert!(test_dir.contains().file_named("f1"));
}

#[test]
fn does_not_contain() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let file = File::from((temp.env().path().join("f9"), "Test content for f1."));

    assert!(!test_dir.contains().file(&file));
}

#[test]
fn does_not_contain_deep() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let file = File::from((temp.env().path().join("f1"), "Test content foflakjhsdf."));

    assert!(!test_dir.contains().deep_file(&file).unwrap());
}

#[test]
fn does_not_contain_named() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    assert!(!test_dir.contains().file_named("notafile"));
}
