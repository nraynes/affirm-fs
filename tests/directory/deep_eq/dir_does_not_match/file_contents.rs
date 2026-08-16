use affirm_fs::{DirStructure, Directory};

use crate::resources::init_temp_env;

#[test]
fn of_child_file_dont_match() {
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
        .dir("d3", |d| {
            d.file_with_contents("d3f1", "Test content for d3f1.")
                .build()
        })
        .file_with_contents("f1", "Test anticontent for f1.")
        .file_with_contents("f2", "Test content for f2.")
        .file_with_contents("f3", "Test content for f3.")
        .build();

    assert!(!test_dir.deep_eq().dir(&dir_structure).unwrap());
}

#[test]
fn of_inner_child_file_dont_match() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| {
                d.file_with_contents("d1d1f1", "Test anticontent for d1d1f1.")
                    .build()
            })
            .build()
        })
        .dir("d2", |d| {
            d.file_with_contents("d2f1", "Test content for d2f1.")
                .build()
        })
        .dir("d3", |d| {
            d.file_with_contents("d3f1", "Test content for d3f1.")
                .build()
        })
        .file_with_contents("f1", "Test content for f1.")
        .file_with_contents("f2", "Test content for f2.")
        .file_with_contents("f3", "Test content for f3.")
        .build();

    assert!(!test_dir.deep_eq().dir(&dir_structure).unwrap());
}

#[test]
fn of_many_child_files_dont_match() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| {
                d.file_with_contents("d1d1f1", "Test anticontent for d1d1f1.")
                    .build()
            })
            .build()
        })
        .dir("d2", |d| {
            d.file_with_contents("d2f1", "Test conteddd8900nt for d2f1.")
                .build()
        })
        .dir("d3", |d| {
            d.file_with_contents("d3f1", "Test content for d3f1.")
                .build()
        })
        .file_with_contents("f1", "Test content for f1.")
        .file_with_contents("f2", "Test anticontent for f2.")
        .file_with_contents("f3", "Test anticontent ffffor f3.")
        .build();

    assert!(!test_dir.deep_eq().dir(&dir_structure).unwrap());
}

#[test]
fn of_child_file_is_empty() {
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
        .dir("d3", |d| {
            d.file_with_contents("d3f1", "Test content for d3f1.")
                .build()
        })
        .file_with_contents("f1", "")
        .file_with_contents("f2", "Test content for f2.")
        .file_with_contents("f3", "Test content for f3.")
        .build();

    assert!(!test_dir.deep_eq().dir(&dir_structure).unwrap());
}

#[test]
fn of_inner_child_file_is_empty() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| d.file_with_contents("d1d1f1", "").build())
                .build()
        })
        .dir("d2", |d| {
            d.file_with_contents("d2f1", "Test content for d2f1.")
                .build()
        })
        .dir("d3", |d| {
            d.file_with_contents("d3f1", "Test content for d3f1.")
                .build()
        })
        .file_with_contents("f1", "Test content for f1.")
        .file_with_contents("f2", "Test content for f2.")
        .file_with_contents("f3", "Test content for f3.")
        .build();

    assert!(!test_dir.deep_eq().dir(&dir_structure).unwrap());
}

#[test]
fn of_many_child_files_are_empty() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| d.file_with_contents("d1d1f1", "").build())
                .build()
        })
        .dir("d2", |d| d.file_with_contents("d2f1", "").build())
        .dir("d3", |d| {
            d.file_with_contents("d3f1", "Test content for d3f1.")
                .build()
        })
        .file_with_contents("f1", "Test content for f1.")
        .file_with_contents("f2", "")
        .file_with_contents("f3", "")
        .build();

    assert!(!test_dir.deep_eq().dir(&dir_structure).unwrap());
}

#[test]
fn of_many_child_files_dont_match_and_are_empty() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| {
                d.file_with_contents("d1d1f1", "Test anticontent fffor d1d1f1.")
                    .build()
            })
            .build()
        })
        .dir("d2", |d| {
            d.file_with_contents("d2f1", "Test conteddd8900nt for d2f1.")
                .build()
        })
        .dir("d3", |d| d.file_with_contents("d3f1", "").build())
        .file_with_contents("f1", "Test content for f1.")
        .file_with_contents("f2", "")
        .file_with_contents("f3", "Test anticontent ffffor f3.")
        .build();

    assert!(!test_dir.deep_eq().dir(&dir_structure).unwrap());
}
