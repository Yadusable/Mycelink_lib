use crate::db::actions::tenant_actions::Tenant;
use crate::db::actions::Protocol;
use crate::db::db_connector::DBConnector;
use crate::model::chat::Chat;
use crate::model::chat_config::ChatConfig;
use crate::model::messenger_service::{MessengerService, PollableService};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{SqliteArgumentValue, SqliteTypeInfo};
use sqlx::{Decode, Encode, Row, Sqlite, Type};
use std::sync::Arc;

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

impl DBConnector<Tenant> {
    pub async fn list_chats<'a>(
        &'a self,
        messenger_services: &'a [PollableService],
    ) -> impl Stream<Item = Result<Chat, sqlx::Error>> + 'a {
        let query =
            sqlx::query("SELECT id, display_name, protocol FROM chat_ids WHERE tenant = ?;")
                .bind(self.tenant());

        query.fetch(self.pool().await).map(move |e| {
            e.and_then(|row| {
                Ok(Chat {
                    id: row.get("id"),
                    display_name: row.get("display_name"),
                    alt_name: None,
                    message_service: messenger_services
                        .iter()
                        .map(|e| e.service())
                        .find(|e| {
                            Into::<&str>::into(e.protocol()) == row.get::<&str, &str>("protocol")
                        })
                        .ok_or(sqlx::Error::RowNotFound)?,
                    db_connector: self,
                })
            })
        })
    }

    pub async fn list_protocol_chats<'a>(
        &'a self,
        messenger_service: &'a dyn MessengerService,
    ) -> impl Stream<Item = sqlx::Result<Chat>> + 'a {
        let query = sqlx::query(
            "SELECT id, display_name, protocol FROM chat_ids WHERE protocol = ? AND tenant = ?;",
        )
        .bind(messenger_service.protocol())
        .bind(self.tenant());

        query.fetch(self.pool().await).map(move |e| {
            e.and_then(|row| {
                Ok(Chat {
                    id: row.get("id"),
                    display_name: row.get("display_name"),
                    alt_name: None,
                    message_service: messenger_service,
                    db_connector: self,
                })
            })
        })
    }

    pub async fn get_chat_config(&self, chat_id: ChatId) -> sqlx::Result<Option<ChatConfig>> {
        let query = sqlx::query("SELECT protocol_config FROM chat_ids WHERE id = ?").bind(chat_id);

        let row = query.fetch_optional(self.pool().await).await?;
        Ok(row.map(|row| {
            ciborium::from_reader(row.get::<Vec<u8>, &str>("protocol_config").as_slice()).unwrap()
        }))
    }
}
