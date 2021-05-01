use crate::domain::problem::solution_state::SolutionState;
use crate::domain::Solution;

mod solution_state;

pub(crate) struct Problem {
    target: u16,
    bucket_sizes: Vec<u16>,
}

impl Problem {
    pub(crate) fn new(target: u16, bucket_sizes: Vec<u16>) -> Self {
        Problem {
            target,
            bucket_sizes,
        }
    }

    pub(crate) fn validate_solution(&self, solution: &Solution) -> SolutionValidity {
        let mut solution_state = SolutionState::new(self.bucket_sizes.clone());
        solution
            .actions()
            .iter()
            .try_for_each(|action| solution_state.apply_action(action))
            .unwrap();

        if solution_state.has_measurement(self.target) {
            SolutionValidity::Valid
        } else {
            SolutionValidity::Invalid
        }
    }
}

pub(crate) enum SolutionValidity {
    Valid,
    Invalid,
}
