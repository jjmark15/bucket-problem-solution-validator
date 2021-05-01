use crate::domain::{PourAction, Solution, SolutionAction};

pub(crate) struct PourActionDto {
    source_bucket: usize,
    target_bucket: usize,
}

impl PourActionDto {
    pub(crate) fn new(source_bucket: usize, target_bucket: usize) -> Self {
        PourActionDto {
            source_bucket,
            target_bucket,
        }
    }
}

pub(crate) enum SolutionActionDto {
    Pour(PourActionDto),
    Empty(usize),
    Fill(usize),
}

pub(crate) struct SolutionDto {
    actions: Vec<SolutionActionDto>,
}

impl SolutionDto {
    pub(crate) fn new(actions: Vec<SolutionActionDto>) -> Self {
        SolutionDto { actions }
    }
}

impl From<SolutionDto> for Solution {
    fn from(solution: SolutionDto) -> Self {
        Solution::new(
            solution
                .actions
                .into_iter()
                .map(SolutionAction::from)
                .collect(),
        )
    }
}

impl From<SolutionActionDto> for SolutionAction {
    fn from(action: SolutionActionDto) -> Self {
        match action {
            SolutionActionDto::Pour(pour_action) => SolutionAction::Pour(pour_action.into()),
            SolutionActionDto::Empty(index) => SolutionAction::Empty(index.into()),
            SolutionActionDto::Fill(index) => SolutionAction::Fill(index.into()),
        }
    }
}

impl From<PourActionDto> for PourAction {
    fn from(pour_action: PourActionDto) -> Self {
        PourAction::new(
            pour_action.source_bucket.into(),
            pour_action.target_bucket.into(),
        )
    }
}
