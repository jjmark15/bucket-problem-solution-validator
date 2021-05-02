use std::path::PathBuf;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::str::contains;

use crate::helpers::{
    non_trivial_problem, solution_with_incorrect_bucket_index, write_problem_to_file,
    write_solution_to_file, CliCommandBuilder,
};

#[test]
fn fails_to_validate_solution_using_invalid_bucket_index() {
    let temp = TempDir::new().unwrap();
    let problem_file_path: PathBuf = temp.child("problem").to_path_buf();
    let solution_file_path: PathBuf = temp.child("solution").to_path_buf();
    write_problem_to_file(problem_file_path.as_path(), &non_trivial_problem()).unwrap();
    write_solution_to_file(
        solution_file_path.as_path(),
        &solution_with_incorrect_bucket_index(),
    )
    .unwrap();

    let assert = CliCommandBuilder::new()
        .with_problem_file(problem_file_path.as_path())
        .with_solution_file(solution_file_path.as_path())
        .assert();

    assert
        .failure()
        .stderr(contains("Attempted use of invalid bucket index (3)"));
}
