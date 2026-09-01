use affirm_fs::{DirStructure, Directory};

use crate::resources::init_temp_env;

#[test]
fn works() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let dir_structure = DirStructure::new(temp.env().path())
        .dir("d1", |d| {
            d.dir("d1d1", |d| d.file_with_contents("d1d1f1", "").build())
                .build()
        })
        .dir("d2", |d| d.file_with_contents("d2f1", "").build())
        .dir("d3", |d| d.file_with_contents("d3f1", "").build())
        .file_with_contents("f1", "")
        .file_with_contents("f2", "")
        .file_with_contents("f3", "")
        .build();

    assert!(test_dir.eq().dir(&dir_structure));
    assert!(!test_dir.deep_eq().dir(&dir_structure).unwrap());

    let dir_structure = dir_structure.take_and_acquire_contents().unwrap();

    assert!(test_dir.deep_eq().dir(&dir_structure).unwrap());
}
