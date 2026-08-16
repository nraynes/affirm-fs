use affirm_fs::{DirStructure, Directory};

use crate::resources::init_temp_env;

#[test]
fn does_contain() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| d.file("d1d1f1").build()).build()
        })
        .dir("d2", |d| d.file("d2f1").build())
        .file("f1")
        .build();

    assert!(test_dir.contains().structure(&dir_structure));
}

#[test]
fn does_contain_deep() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| {
                d.file_with_contents("d1d1f1", "Test content for d1d1f1.")
                    .build()
            })
            .build()
        })
        .dir("d2", |d| {
            d.file_with_contents("d2f1", "Test content for d2f1.")
                .build()
        })
        .file_with_contents("f1", "Test content for f1.")
        .build();

    assert!(test_dir.contains().deep_structure(&dir_structure).unwrap());
}

#[test]
fn does_not_contain() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d6", |d| {
            d.dir("d19", |d| d.file("d1d1f1").build())
                .dir("d133", |d| d.file("d1f99").build())
                .build()
        })
        .build();

    assert!(!test_dir.contains().structure(&dir_structure));
}

#[test]
fn does_not_contain_deep() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| {
                d.file_with_contents("d1d1f1", "aliuhwfew890f").build()
            })
            .build()
        })
        .dir("d2", |d| {
            d.file_with_contents("d2f1", "asldkfjhasdgf0").build()
        })
        .file_with_contents("f1", "234u90348kdkk")
        .build();

    assert!(!test_dir.contains().deep_structure(&dir_structure).unwrap());
}
