use serde::Deserialize;

use crate::application::ProblemDto;

#[derive(Debug, Deserialize)]
pub(crate) struct YamlProblem {
    target: u16,
    bucket_sizes: Vec<u16>,
}

impl From<YamlProblem> for ProblemDto {
    fn from(problem: YamlProblem) -> Self {
        ProblemDto::new(problem.target, problem.bucket_sizes)
    }
}
