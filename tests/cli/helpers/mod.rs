use std::path::Path;

use crate::helpers::models::{PourAction, Problem, Solution, SolutionAction};
use assert_cmd::Command;

pub(crate) mod models;

pub(crate) fn write_solution_to_file(file_path: &Path, solution: &Solution) -> std::io::Result<()> {
    std::fs::write(file_path, serde_yaml::to_string(solution).unwrap())
}

pub(crate) fn write_problem_to_file(file_path: &Path, problem: &Problem) -> std::io::Result<()> {
    std::fs::write(file_path, serde_yaml::to_string(problem).unwrap())
}

pub(crate) fn fill(bucket_index: usize) -> SolutionAction {
    SolutionAction::Fill(bucket_index)
}

pub(crate) fn pour(source_bucket_index: usize, target_bucket_index: usize) -> SolutionAction {
    SolutionAction::Pour(PourAction::new(source_bucket_index, target_bucket_index))
}

pub(crate) fn non_trivial_problem() -> Problem {
    Problem::new(6, vec![12, 8, 5])
}

pub(crate) fn valid_solution() -> Solution {
    Solution::new(vec![
        fill(0),
        pour(0, 1),
        pour(1, 2),
        pour(2, 0),
        pour(1, 2),
        pour(0, 1),
        pour(1, 2),
    ])
}

pub(crate) fn validator_command() -> Command {
    Command::cargo_bin("validator").unwrap()
}
