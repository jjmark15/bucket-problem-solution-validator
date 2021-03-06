use std::path::PathBuf;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::str::contains;

use crate::helpers::{
    non_trivial_problem, valid_solution, write_problem_to_file, write_solution_to_file,
    CliCommandBuilder,
};

#[test]
fn validates_correct_solution() {
    let temp = TempDir::new().unwrap();
    let problem_file_path: PathBuf = temp.child("problem").to_path_buf();
    let solution_file_path: PathBuf = temp.child("solution").to_path_buf();
    write_problem_to_file(problem_file_path.as_path(), &non_trivial_problem()).unwrap();
    write_solution_to_file(solution_file_path.as_path(), &valid_solution()).unwrap();

    let assert = CliCommandBuilder::new()
        .with_problem_file(problem_file_path.as_path())
        .with_solution_file(solution_file_path.as_path())
        .assert();

    assert.success().stdout(contains("Solution is valid"));
}
