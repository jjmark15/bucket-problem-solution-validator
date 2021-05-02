use std::fs::File;
use std::path::Path;

use crate::ports::persistence::yaml::error::ProblemFormatError;
use crate::ports::persistence::yaml::problem::YamlProblem;

pub(crate) struct YamlProblemReaderAdapter {}

impl YamlProblemReaderAdapter {
    pub(crate) fn new() -> Self {
        YamlProblemReaderAdapter {}
    }

    pub(crate) fn read(&self, path: &Path) -> Result<YamlProblem, ReadYamlProblemError> {
        let file = File::open(path).map_err(ReadYamlProblemError::from)?;
        serde_yaml::from_reader(file).map_err(ReadYamlProblemError::from)
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ReadYamlProblemError {
    #[error("Could not access problem file")]
    FileAccess,
    #[error(transparent)]
    Format(#[from] ProblemFormatError),
}

impl From<serde_yaml::Error> for ReadYamlProblemError {
    fn from(_err: serde_yaml::Error) -> Self {
        ProblemFormatError.into()
    }
}

impl From<std::io::Error> for ReadYamlProblemError {
    fn from(_err: std::io::Error) -> Self {
        ReadYamlProblemError::FileAccess
    }
}
