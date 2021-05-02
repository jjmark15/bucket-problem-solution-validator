use std::path::PathBuf;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::str::contains;

use crate::helpers::{non_trivial_problem, write_problem_to_file, CliCommandBuilder};

#[test]
fn fails_to_validate_solution_given_missing_solution_file() {
    let temp = TempDir::new().unwrap();
    let problem_file_path: PathBuf = temp.child("problem").to_path_buf();
    let solution_file_path: PathBuf = temp.to_path_buf().join("solution");
    write_problem_to_file(problem_file_path.as_path(), &non_trivial_problem()).unwrap();

    let assert = CliCommandBuilder::new()
        .with_problem_file(problem_file_path.as_path())
        .with_solution_file(solution_file_path.as_path())
        .assert();

    assert
        .failure()
        .stderr(contains("Could not access solution file"));
}
