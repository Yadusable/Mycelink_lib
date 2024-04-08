use crate::mycelink::mycelink_account::MycelinkAccount;
use serde::{Deserialize, Serialize};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::SqliteTypeInfo;
use sqlx::{Decode, Encode, Sqlite, Type};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{FromStr, ParseBoolError};
use std::string::ParseError;

#[derive(Serialize, Deserialize)]
pub enum ProtocolConfig {
    Mycelink { account: MycelinkAccount },
}

impl TryFrom<ProtocolConfig> for MycelinkAccount {
    type Error = ();

    fn try_from(value: ProtocolConfig) -> Result<Self, Self::Error> {
        if let ProtocolConfig::Mycelink { account } = value {
            Ok(account)
        } else {
            Err(())
        }
    }
}

#[derive(Copy, Clone)]
pub enum Protocol {
    Mycelink,
}

impl From<Protocol> for &str {
    fn from(value: Protocol) -> Self {
        match value {
            Protocol::Mycelink => "Mycelink",
        }
    }
}

impl FromStr for Protocol {
    type Err = ParseProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Mycelink" => Ok(Protocol::Mycelink),
            default => Err(ParseProtocolError(default.into())),
        }
    }
}

impl Encode<'_, Sqlite> for Protocol {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        <&str as Encode<Sqlite>>::encode_by_ref(&(*self).into(), buf)
    }
}

impl Type<Sqlite> for Protocol {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }
}

impl Decode<'_, Sqlite> for Protocol {
    fn decode(value: <Sqlite as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let s = <&str as Decode<Sqlite>>::decode(value)?;
        s.parse().map_err(|e| Box::new(e) as BoxDynError)
    }
}

#[derive(Debug)]
pub struct ParseProtocolError(Box<str>);

impl Display for ParseProtocolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a valid protocol", self.0)
    }
}

impl Error for ParseProtocolError {}
