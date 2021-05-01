use crate::application::ValidityDto;

pub(crate) enum CliValidity {
    Valid,
    Invalid,
}

impl CliValidity {
    pub(crate) fn description(&self) -> &str {
        match self {
            CliValidity::Valid => "valid",
            CliValidity::Invalid => "invalid",
        }
    }
}

impl From<ValidityDto> for CliValidity {
    fn from(validity: ValidityDto) -> Self {
        match validity {
            ValidityDto::Valid => CliValidity::Valid,
            ValidityDto::Invalid => CliValidity::Invalid,
        }
    }
}
