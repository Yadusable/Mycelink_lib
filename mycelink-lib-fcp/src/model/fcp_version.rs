use crate::decode_error::DecodeError;
use crate::model::fields::Field;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            _default => Err(DecodeError::ParseError(
                format!("Failed parsing {value} as a valid FCP version").into(),
            )),
        }
    }
}

impl TryFrom<&Field> for FCPVersion {
    type Error = DecodeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        if value.key() == "FCPVersion" {
            Ok(value.value().try_into()?)
        } else {
            Err(DecodeError::MissingField("FCPVersion".into()))
        }
    }
}
