use std::path::PathBuf;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::str::contains;

use crate::helpers::{valid_solution, write_solution_to_file, CliCommandBuilder};

#[test]
fn fails_to_validate_solution_given_missing_problem_file() {
    let temp = TempDir::new().unwrap();
    let problem_file_path: PathBuf = temp.to_path_buf().join("problem");
    let solution_file_path: PathBuf = temp.child("solution").to_path_buf();
    write_solution_to_file(solution_file_path.as_path(), &valid_solution()).unwrap();

    let assert = CliCommandBuilder::new()
        .with_problem_file(problem_file_path.as_path())
        .with_solution_file(solution_file_path.as_path())
        .assert();

    assert
        .failure()
        .stderr(contains("Could not access problem file"));
}
