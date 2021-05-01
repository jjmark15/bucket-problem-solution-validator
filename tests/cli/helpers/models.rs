use serde::Serialize;

#[derive(Debug, Serialize)]
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
}

#[derive(Debug, Serialize)]
pub(crate) struct PourAction {
    source_bucket: usize,
    target_bucket: usize,
}

impl PourAction {
    pub(crate) fn new(source_bucket: usize, target_bucket: usize) -> Self {
        PourAction {
            source_bucket,
            target_bucket,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) enum SolutionAction {
    Pour(PourAction),
    Fill(usize),
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct Solution {
    actions: Vec<SolutionAction>,
}

impl Solution {
    pub(crate) fn new(actions: Vec<SolutionAction>) -> Self {
        Solution { actions }
    }
}
