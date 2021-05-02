use std::path::Path;

use assert_cmd::assert::Assert;
use assert_cmd::Command;

pub(crate) struct CliCommandBuilder {
    cmd: Command,
}

impl CliCommandBuilder {
    pub(crate) fn new() -> Self {
        CliCommandBuilder {
            cmd: Command::cargo_bin("validator").unwrap(),
        }
    }

    pub(crate) fn with_problem_file(mut self, file_path: &Path) -> Self {
        self.cmd
            .args(&["--problem-file", file_path.as_os_str().to_str().unwrap()]);
        self
    }

    pub(crate) fn with_solution_file(mut self, file_path: &Path) -> Self {
        self.cmd
            .args(&["--solution-file", file_path.as_os_str().to_str().unwrap()]);
        self
    }

    pub(crate) fn assert(mut self) -> Assert {
        self.cmd.assert()
    }
}
