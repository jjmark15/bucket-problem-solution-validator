use crate::domain::{Bucket, BucketIndex, PourAction, SolutionAction};

pub(crate) struct SolutionState {
    buckets: Vec<Bucket>,
}

impl SolutionState {
    pub(crate) fn new(bucket_sizes: Vec<u16>) -> Self {
        let buckets = bucket_sizes
            .iter()
            .map(|capacity| Bucket::new(*capacity))
            .collect();
        SolutionState { buckets }
    }

    fn get_bucket_at_index(
        &mut self,
        index: BucketIndex,
    ) -> Result<&Bucket, InvalidBucketIndexError> {
        self.buckets
            .get(index.0)
            .ok_or(InvalidBucketIndexError(index))
    }

    fn update_bucket_at_index(
        &mut self,
        index: BucketIndex,
        bucket: Bucket,
    ) -> Result<(), InvalidBucketIndexError> {
        self.buckets
            .get(index.0)
            .ok_or(InvalidBucketIndexError(index))?;
        self.buckets[index.0] = bucket;

        Ok(())
    }

    fn pour(&mut self, pour_action: &PourAction) -> Result<(), InvalidBucketIndexError> {
        let mut source_bucket = *self.get_bucket_at_index(pour_action.source_bucket())?;
        let mut target_bucket = *self.get_bucket_at_index(pour_action.target_bucket())?;
        source_bucket.pour_into(&mut target_bucket);
        self.update_bucket_at_index(pour_action.source_bucket(), source_bucket)?;
        self.update_bucket_at_index(pour_action.target_bucket(), target_bucket)?;
        Ok(())
    }

    fn empty_bucket_at_index(&mut self, index: BucketIndex) -> Result<(), InvalidBucketIndexError> {
        let mut bucket = *self.get_bucket_at_index(index)?;
        bucket.empty();
        self.update_bucket_at_index(index, bucket)?;
        Ok(())
    }

    fn fill_bucket_at_index(&mut self, index: BucketIndex) -> Result<(), InvalidBucketIndexError> {
        let mut bucket = *self.get_bucket_at_index(index)?;
        bucket.fill();
        self.update_bucket_at_index(index, bucket)?;
        Ok(())
    }

    pub(crate) fn apply_action(
        &mut self,
        action: &SolutionAction,
    ) -> Result<(), ApplySolutionActionError> {
        match action {
            SolutionAction::Pour(pour_action) => self
                .pour(pour_action)
                .map_err(ApplySolutionActionError::from),
            SolutionAction::Empty(index) => self
                .empty_bucket_at_index(*index)
                .map_err(ApplySolutionActionError::from),
            SolutionAction::Fill(index) => self
                .fill_bucket_at_index(*index)
                .map_err(ApplySolutionActionError::from),
        }
    }

    pub(crate) fn has_measurement(&self, measurement: u16) -> bool {
        self.buckets
            .iter()
            .any(|bucket| bucket.measurement() == measurement)
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ApplySolutionActionError {
    #[error(transparent)]
    InvalidIndex(#[from] InvalidBucketIndexError),
}

#[derive(Debug, thiserror::Error)]
#[error("Attempted to perform action against invalid bucket of index '{0}'")]
pub(crate) struct InvalidBucketIndexError(BucketIndex);
