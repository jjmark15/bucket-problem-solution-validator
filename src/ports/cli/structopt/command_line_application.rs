use std::error::Error;
use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;

use crate::application::ApplicationService;
use crate::ports::cli::structopt::validity::CliValidity;
use crate::ports::persistence::yaml::{YamlProblemReaderAdapter, YamlSolutionReaderAdapter};

#[derive(StructOpt, Debug)]
#[structopt(name = "validator")]
struct ValidatorOptions {
    /// YAML problem file path
    #[structopt(short, long, parse(from_os_str))]
    problem_file: PathBuf,

    /// YAML solution file path
    #[structopt(short, long, parse(from_os_str))]
    solution_file: PathBuf,
}

fn unwrap_or_exit_app_with_error_message<U, E: Error>(result: Result<U, E>) -> U {
    match result {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

pub(crate) fn run_cli<AS: ApplicationService>(application_service: &AS) {
    let solution_reader = YamlSolutionReaderAdapter::new();
    let problem_reader = YamlProblemReaderAdapter::new();
    let opt: ValidatorOptions = ValidatorOptions::from_args();

    let solution =
        unwrap_or_exit_app_with_error_message(solution_reader.read(opt.solution_file.as_path()));
    let problem =
        unwrap_or_exit_app_with_error_message(problem_reader.read(opt.problem_file.as_path()));

    let validity: CliValidity = application_service
        .validate_solution(problem.into(), solution.into())
        .into();

    println!("Solution is {}", validity.description());
}
