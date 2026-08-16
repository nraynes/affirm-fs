use affirm_fs::{DirStructure, Directory};

use crate::resources::init_temp_env;

#[test]
fn dir() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| d.file("d1d1f1").build()).build()
        })
        .dir("d2", |d| d.file("d2f1").build())
        .dir("d3", |d| d.file("d3f1").build())
        .file("f1")
        .file("f2")
        .file("f3")
        .build();

    assert!(test_dir.eq().dir(&dir_structure));
    assert!(!test_dir.ne().dir(&dir_structure));
}
