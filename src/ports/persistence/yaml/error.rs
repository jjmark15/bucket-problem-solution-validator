#[derive(Debug, thiserror::Error)]
#[error("Could not access file")]
pub(crate) struct FileAccessError;

#[derive(Debug, thiserror::Error)]
#[error("Could not read solution file format")]
pub(crate) struct SolutionFormatError;

#[derive(Debug, thiserror::Error)]
#[error("Could not read problem file format")]
pub(crate) struct ProblemFormatError;
