use std::fmt::Formatter;

#[derive(Debug)]
pub(crate) struct PourAction {
    source_bucket: BucketIndex,
    target_bucket: BucketIndex,
}

impl PourAction {
    pub(crate) fn new(source_bucket: BucketIndex, target_bucket: BucketIndex) -> Self {
        PourAction {
            source_bucket,
            target_bucket,
        }
    }

    pub(crate) fn source_bucket(&self) -> BucketIndex {
        self.source_bucket
    }

    pub(crate) fn target_bucket(&self) -> BucketIndex {
        self.target_bucket
    }
}

pub(crate) enum SolutionAction {
    Pour(PourAction),
    Empty(BucketIndex),
    Fill(BucketIndex),
}

pub(crate) struct Solution {
    actions: Vec<SolutionAction>,
}

impl Solution {
    pub(crate) fn new(actions: Vec<SolutionAction>) -> Self {
        Solution { actions }
    }

    pub(crate) fn actions(&self) -> &Vec<SolutionAction> {
        &self.actions
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct BucketIndex(pub(crate) usize);

impl std::fmt::Display for BucketIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for BucketIndex {
    fn from(index: usize) -> Self {
        BucketIndex(index)
    }
}
