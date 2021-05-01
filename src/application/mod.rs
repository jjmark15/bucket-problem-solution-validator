pub(crate) use problem::*;
pub(crate) use solution::*;
pub(crate) use validity::*;

use crate::domain::Problem;

mod problem;
mod solution;
mod validity;

pub(crate) trait ApplicationService {
    fn validate_solution(&self, problem: ProblemDto, solution: SolutionDto) -> ValidityDto;
}

pub(crate) struct ApplicationServiceImpl;

impl ApplicationServiceImpl {
    pub(crate) fn new() -> Self {
        ApplicationServiceImpl
    }
}

impl ApplicationService for ApplicationServiceImpl {
    fn validate_solution(&self, problem: ProblemDto, solution: SolutionDto) -> ValidityDto {
        Problem::from(problem)
            .validate_solution(&solution.into())
            .into()
    }
}
