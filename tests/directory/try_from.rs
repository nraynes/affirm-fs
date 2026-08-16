use affirm_fs::Directory;

use crate::resources::{assert_dir_structure, init_temp_env};

#[test]
fn path() {
    let mut temp = init_temp_env();

    let test_dir = Directory::try_from(temp.env().path()).unwrap();
    let temp_env_path = temp.env().path().clone();

    assert_dir_structure(
        temp.env(),
        &test_dir,
        &[
            (
                &temp_env_path.join("d1").join("d1d1").join("d1d1f1"),
                "Test content for d1d1f1.",
            ),
            (
                &temp_env_path.join("d2").join("d2f1"),
                "Test content for d2f1.",
            ),
            (
                &temp_env_path.join("d3").join("d3f1"),
                "Test content for d3f1.",
            ),
            (&temp_env_path.join("f1"), "Test content for f1."),
            (&temp_env_path.join("f2"), "Test content for f2."),
            (&temp_env_path.join("f3"), "Test content for f3."),
        ],
    );
}
