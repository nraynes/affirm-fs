use affirm_fs::{DirStructure, Directory};

use crate::resources::init_temp_env;

#[test]
fn does_contain() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path().join("d1"))
        .dir("d1d1", |d| d.file("d1d1f1").build())
        .build();

    assert!(test_dir.contains().dir(&dir_structure));
}

#[test]
fn does_contain_deep() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path().join("d1"))
        .dir("d1d1", |d| {
            d.file_with_contents("d1d1f1", "Test content for d1d1f1.")
                .build()
        })
        .build();

    assert!(test_dir.contains().deep_dir(&dir_structure).unwrap());
}

#[test]
fn contains_named() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    assert!(test_dir.contains().dir_named("d1"));
}

#[test]
fn does_not_contain() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path().join("notcorrect"))
        .dir("d1d1", |d| d.file("d1d1f1").build())
        .build();

    assert!(!test_dir.contains().dir(&dir_structure));
}

#[test]
fn does_not_contain_deep() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path().join("notcorrect"))
        .dir("d1d1", |d| {
            d.file_with_contents("d1d1f1", "Test content for d1d1f1.")
                .build()
        })
        .build();

    assert!(!test_dir.contains().deep_dir(&dir_structure).unwrap());
}

#[test]
fn does_not_contain_named() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    assert!(!test_dir.contains().dir_named("notafolder"));
}
