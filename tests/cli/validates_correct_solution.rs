use std::path::PathBuf;

use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::str::contains;

use crate::helpers::models::{Problem, Solution};
use crate::helpers::{fill, pour, write_problem_to_file, write_solution_to_file};

#[test]
fn validates_correct_solution() {
    let temp = TempDir::new().unwrap();
    let problem_file_path: PathBuf = temp.child("problem").to_path_buf();
    let solution_file_path: PathBuf = temp.child("solution").to_path_buf();
    let problem = Problem::new(6, vec![12, 8, 5]);
    let solution = Solution::new(vec![
        fill(0),
        pour(0, 1),
        pour(1, 2),
        pour(2, 0),
        pour(1, 2),
        pour(0, 1),
        pour(1, 2),
    ]);
    let mut cmd = Command::cargo_bin("validator").unwrap();
    write_problem_to_file(problem_file_path.as_path(), &problem).unwrap();
    write_solution_to_file(solution_file_path.as_path(), &solution).unwrap();

    let assert = cmd
        .args(&[
            "--problem-file",
            problem_file_path.as_os_str().to_str().unwrap(),
            "--solution-file",
            solution_file_path.as_os_str().to_str().unwrap(),
        ])
        .assert();

    assert.success().stdout(contains("Solution is valid"));
}
