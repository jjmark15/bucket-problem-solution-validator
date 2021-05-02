use std::fs::File;
use std::path::Path;

use crate::ports::persistence::yaml::error::SolutionFormatError;
use crate::ports::persistence::yaml::solution::YamlSolution;

pub(crate) struct YamlSolutionReaderAdapter;

impl YamlSolutionReaderAdapter {
    pub(crate) fn new() -> Self {
        YamlSolutionReaderAdapter
    }

    pub(crate) fn read(&self, path: &Path) -> Result<YamlSolution, ReadYamlSolutionError> {
        let file = File::open(path).map_err(ReadYamlSolutionError::from)?;
        serde_yaml::from_reader(file).map_err(ReadYamlSolutionError::from)
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ReadYamlSolutionError {
    #[error("Could not access solution file")]
    FileAccess,
    #[error(transparent)]
    Format(#[from] SolutionFormatError),
}

impl From<serde_yaml::Error> for ReadYamlSolutionError {
    fn from(_err: serde_yaml::Error) -> Self {
        SolutionFormatError.into()
    }
}

impl From<std::io::Error> for ReadYamlSolutionError {
    fn from(_err: std::io::Error) -> Self {
        ReadYamlSolutionError::FileAccess
    }
}
