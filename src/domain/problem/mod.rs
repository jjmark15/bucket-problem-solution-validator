use crate::domain::problem::solution_state::{
    ApplySolutionActionError, InvalidBucketIndexError, SolutionState,
};
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

    pub(crate) fn validate_solution(
        &self,
        solution: &Solution,
    ) -> Result<SolutionValidity, ValidateSolutionError> {
        let mut solution_state = SolutionState::new(self.bucket_sizes.clone());
        solution
            .actions()
            .iter()
            .try_for_each(|action| solution_state.apply_action(action))?;

        if solution_state.has_measurement(self.target) {
            Ok(SolutionValidity::Valid)
        } else {
            Ok(SolutionValidity::Invalid)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ValidateSolutionError {
    #[error(transparent)]
    InvalidIndex(#[from] InvalidBucketIndexError),
}

impl From<ApplySolutionActionError> for ValidateSolutionError {
    fn from(err: ApplySolutionActionError) -> Self {
        match err {
            ApplySolutionActionError::InvalidIndex(invalid_index_err) => invalid_index_err.into(),
        }
    }
}

pub(crate) enum SolutionValidity {
    Valid,
    Invalid,
}
