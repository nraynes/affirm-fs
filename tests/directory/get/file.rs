use affirm_fs::Directory;

use crate::resources::init_temp_env;

#[test]
fn exists() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let inner_file = test_dir.file(temp.env().path().join("f1")).unwrap();

    assert_eq!(inner_file.path(), &temp.env().path().join("f1"));
}

#[test]
fn inner_exists() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let inner_file = test_dir
        .file(temp.env().path().join("d1").join("d1d1").join("d1d1f1"))
        .unwrap();

    assert_eq!(
        inner_file.path(),
        &temp.env().path().join("d1").join("d1d1").join("d1d1f1")
    );
}

#[test]
fn does_not_exist() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let inner_file = test_dir.file(temp.env().path().join("notavailable"));

    assert!(inner_file.is_none());
}

#[test]
fn inner_does_not_exist() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();

    let inner_file = test_dir.file(
        temp.env()
            .path()
            .join("d1")
            .join("d1d1")
            .join("notavailable"),
    );

    assert!(inner_file.is_none());
}
