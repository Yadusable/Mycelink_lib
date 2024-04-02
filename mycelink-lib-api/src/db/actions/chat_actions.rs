use crate::db::actions::tenant_actions::Tenant;
use crate::db::actions::Protocol;
use crate::db::db_connector::DBConnector;
use crate::model::chat_config::ChatConfig;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{SqliteArgumentValue, SqliteTypeInfo};
use sqlx::{Decode, Encode, Row, Sqlite, Type};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ChatId(i64);
impl Decode<'_, Sqlite> for ChatId {
    fn decode(value: <Sqlite as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let id = <i64 as Decode<Sqlite>>::decode(value)?;
        Ok(ChatId(id))
    }
}
impl Type<Sqlite> for ChatId {
    fn type_info() -> SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
}
impl Encode<'_, Sqlite> for ChatId {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        buf.push(SqliteArgumentValue::Int64(self.0));
        IsNull::No
    }
}

pub struct ChatSchema {
    id: ChatId,
    display_name: Box<str>,
    protocol_config: Box<[u8]>,
}

impl DBConnector<Tenant> {
    pub async fn list_chats(
        &self,
        protocol: Protocol,
    ) -> Result<impl Stream<Item = Result<ChatSchema, sqlx::Error>> + '_, sqlx::Error> {
        let query = sqlx::query(
            "SELECT (id, display_name, protocol_config) FROM chat_ids WHERE protocol = ? AND tenant = ?;",
        )
            .bind(protocol)
            .bind(self.tenant());

        Ok(query.fetch(self.pool().await).map(|e| {
            e.map(|e| {
                let id = e.try_get("id").unwrap();
                let display_name = e.try_get("display_name").unwrap();
                let protocol_config = e.try_get("protocol_config").unwrap();
                ChatSchema {
                    id,
                    display_name,
                    protocol_config,
                }
            })
        }))
    }

    pub async fn get_chat_config(&self, chat_id: ChatId) -> sqlx::Result<Option<ChatConfig>> {
        let query = sqlx::query("SELECT protocol_config FROM chat_ids WHERE id = ?").bind(chat_id);

        let row = query.fetch_optional(self.pool().await).await?;
        Ok(row.map(|row| {
            ciborium::from_reader(row.get::<Vec<u8>, &str>("protocol_config").as_slice()).unwrap()
        }))
    }
}
