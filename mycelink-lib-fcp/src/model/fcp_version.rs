use crate::decode_error::DecodeError;

#[derive(Copy, Clone, Debug)]
pub enum FCPVersion {
    V2_0,
}

impl FCPVersion {
    pub fn name(&self) -> &'static str {
        match self {
            FCPVersion::V2_0 => "2.0",
        }
    }
}

impl TryFrom<&str> for FCPVersion {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "2.0" => Ok(FCPVersion::V2_0),
            _default => Err(DecodeError::ParseError(value.into())),
        }
    }
}
