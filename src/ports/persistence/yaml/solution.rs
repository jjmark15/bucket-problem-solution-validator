use serde::Deserialize;

use crate::application::{PourActionDto, SolutionActionDto, SolutionDto};

#[derive(Debug, Deserialize)]
pub(crate) struct YamlPourAction {
    source_bucket: usize,
    target_bucket: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) enum YamlSolutionAction {
    Pour(YamlPourAction),
    Empty(usize),
    Fill(usize),
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(crate) struct YamlSolution {
    actions: Vec<YamlSolutionAction>,
}

impl From<YamlSolution> for SolutionDto {
    fn from(solution: YamlSolution) -> Self {
        SolutionDto::new(
            solution
                .actions
                .into_iter()
                .map(SolutionActionDto::from)
                .collect(),
        )
    }
}

impl From<YamlSolutionAction> for SolutionActionDto {
    fn from(action: YamlSolutionAction) -> Self {
        match action {
            YamlSolutionAction::Pour(pour_action) => SolutionActionDto::Pour(pour_action.into()),
            YamlSolutionAction::Empty(bucket_index) => SolutionActionDto::Empty(bucket_index),
            YamlSolutionAction::Fill(bucket_index) => SolutionActionDto::Fill(bucket_index),
        }
    }
}

impl From<YamlPourAction> for PourActionDto {
    fn from(pour_action: YamlPourAction) -> Self {
        PourActionDto::new(pour_action.source_bucket, pour_action.target_bucket)
    }
}
