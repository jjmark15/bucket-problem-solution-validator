use crate::domain::SolutionValidity;

pub(crate) enum ValidityDto {
    Valid,
    Invalid,
}

impl From<SolutionValidity> for ValidityDto {
    fn from(validity: SolutionValidity) -> Self {
        match validity {
            SolutionValidity::Valid => ValidityDto::Valid,
            SolutionValidity::Invalid => ValidityDto::Invalid,
        }
    }
}
