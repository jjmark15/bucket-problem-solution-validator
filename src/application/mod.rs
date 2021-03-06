pub(crate) use problem::*;
pub(crate) use solution::*;
pub(crate) use validity::*;

use crate::domain::{Problem, ValidateSolutionError};

mod problem;
mod solution;
mod validity;

pub(crate) trait ApplicationService {
    fn validate_solution(
        &self,
        problem: ProblemDto,
        solution: SolutionDto,
    ) -> Result<ValidityDto, ApplicationValidateSolutionError>;
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct ApplicationValidateSolutionError {
    #[from]
    cause: ValidateSolutionError,
}

pub(crate) struct ApplicationServiceImpl;

impl ApplicationServiceImpl {
    pub(crate) fn new() -> Self {
        ApplicationServiceImpl
    }
}

impl ApplicationService for ApplicationServiceImpl {
    fn validate_solution(
        &self,
        problem: ProblemDto,
        solution: SolutionDto,
    ) -> Result<ValidityDto, ApplicationValidateSolutionError> {
        let validity = Problem::from(problem).validate_solution(&solution.into())?;
        Ok(validity.into())
    }
}
