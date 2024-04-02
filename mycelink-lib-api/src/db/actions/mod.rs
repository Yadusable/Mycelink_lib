use crate::db::db_connector::DatabaseBackend;
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;
use sqlx::sqlite::{SqliteArgumentValue, SqliteTypeInfo};
use sqlx::{Encode, Sqlite, Type};
use std::str::FromStr;

pub mod chat_actions;
pub mod contact_actions;
pub mod message_actions;
pub mod mycelink_account_actions;
pub mod tenant_actions;

#[derive(Copy, Clone, Debug)]
pub enum Protocol {
    Mycelink,
}

impl Protocol {
    fn protocol_identifier(&self) -> &'static str {
        match self {
            Protocol::Mycelink => "Mycelink",
        }
    }
}

impl FromStr for Protocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Mycelink" => Ok(Self::Mycelink),
            _ => Err(()),
        }
    }
}

impl Encode<'_, Sqlite> for Protocol {
    fn encode_by_ref(
        &self,
        buf: &mut <DatabaseBackend as HasArguments<'_>>::ArgumentBuffer,
    ) -> IsNull {
        buf.push(SqliteArgumentValue::Text(self.protocol_identifier().into()));
        IsNull::No
    }
}

impl Type<Sqlite> for Protocol {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }
}
