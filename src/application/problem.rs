use crate::domain::Problem;

pub(crate) struct ProblemDto {
    target: u16,
    bucket_sizes: Vec<u16>,
}

impl ProblemDto {
    pub(crate) fn new(target: u16, bucket_sizes: Vec<u16>) -> Self {
        ProblemDto {
            target,
            bucket_sizes,
        }
    }
}

impl From<ProblemDto> for Problem {
    fn from(problem: ProblemDto) -> Self {
        Problem::new(problem.target, problem.bucket_sizes)
    }
}
