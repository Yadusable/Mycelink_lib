use crate::db::actions::chat_actions::ChatId;
use crate::db::actions::contact_actions::ContactId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::model::contact::ContactDisplay;
use crate::model::message::{Message, ProtocolMessageMeta};
use crate::model::message_types::{MessageContent, MessageType};
use crate::model::protocol_config::Protocol;
use crate::mycelink::protocol::mycelink_chat_message::MycelinkChatMessageId;
use futures::stream::{BoxStream, Map};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{SqliteArgumentValue, SqliteRow, SqliteTypeInfo};
use sqlx::types::Json;
use sqlx::{Decode, Encode, Row, Sqlite, Type};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct MessageId(pub(crate) i64);

impl Decode<'_, Sqlite> for MessageId {
    fn decode(value: <Sqlite as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let id = <i64 as Decode<Sqlite>>::decode(value)?;
        Ok(MessageId(id))
    }
}
impl Type<Sqlite> for MessageId {
    fn type_info() -> SqliteTypeInfo {
        <&[u8] as Type<Sqlite>>::type_info()
    }
}
impl Encode<'_, Sqlite> for MessageId {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        buf.push(SqliteArgumentValue::Int64(self.0));
        IsNull::No
    }
}

pub struct MessageSchema {
    pub message_id: MessageId,
    pub chat_id: ChatId,
    pub contact_id: ContactId,
    pub timestamp: u64,
    pub content: Box<[u8]>,
}

impl DBConnector<Tenant> {
    pub async fn get_message_meta(
        &self,
        message_id: &MessageId,
    ) -> Result<Option<ProtocolMessageMeta>, sqlx::error::Error> {
        let query = sqlx::query(
            "SELECT protocol_message_meta FROM chat_messages WHERE message_id = ? AND tenant = ?",
        )
        .bind(message_id)
        .bind(self.tenant());

        let row = query.fetch_optional(self.pool().await).await?;

        match row {
            None => Ok(None),
            Some(row) => Ok(Some(
                serde_json::from_value(row.get("protocol_message_meta")).unwrap(),
            )),
        }
    }

    pub async fn get_newest_message(&self, chat_id: ChatId) -> sqlx::Result<Message> {
        let query = sqlx::query(
            "SELECT message_id,
                message_content,
                timestamp,
                contact_id,
                display_name,
                alternative_name,
                low_res_profile_picture,
                protocol,
                protocol_message_meta,
                GROUP_CONCAT(reaction_message_id, ',') reactions,
                GROUP_CONCAT(thread_message_id, ',')   threads
            FROM chat_messages
            JOIN contacts ON chat_messages.contact_id = contacts.id AND chat_messages.tenant = contacts.tenant
            LEFT JOIN chat_message_reactions ON chat_message_reactions.root_message_id = chat_messages.message_id
            LEFT JOIN chat_message_threads ON chat_message_threads.root_message_id = chat_messages.message_id
            WHERE chat_id = ?
            AND chat_messages.tenant = ?
            GROUP BY chat_messages.message_id
            ORDER BY timestamp DESC
            LIMIT 1")
            .bind(chat_id)
            .bind(self.tenant());

        query.fetch_one(self.pool().await).await.map(|row| Message {
            sender: ContactDisplay {
                id: row.get("contact_id"),
                display_name: row.get("display_name"),
                alternative_name: row.try_get("alternative_name").ok(),
                protocol: row.get("protocol"),
                preview_profile_picture: row
                    .try_get::<Vec<u8>, &str>("low_res_profile_picture")
                    .ok()
                    .map(|e| e.into()),
            },
            message_id: row.get("message_id"),
            protocol_message_meta: serde_json::from_value(row.get("protocol_message_meta"))
                .unwrap(),
            reactions: row
                .get::<&str, &str>("reactions")
                .split(',')
                .map(|e| MessageId(e.parse().unwrap()))
                .collect(),
            replies: row
                .get::<&str, &str>("threads")
                .split(',')
                .map(|e| MessageId(e.parse().unwrap()))
                .collect(),
            timestamp: row.get::<i64, &str>("timestamp") as u64,
            content: serde_json::from_value(row.get("message_content")).unwrap(),
        })
    }

    pub async fn get_next_messages(
        &self,
        message_id: MessageId,
    ) -> Map<
        BoxStream<sqlx::Result<SqliteRow>>,
        impl FnMut(sqlx::Result<SqliteRow>) -> sqlx::Result<Message>,
    > {
        let query = sqlx::query(
            "SELECT message_id,
                message_content,
                timestamp,
                contact_id,
                display_name,
                alternative_name,
                low_res_profile_picture,
                protocol,
                protocol_message_meta
                GROUP_CONCAT(reaction_message_id, ',') reactions,
                GROUP_CONCAT(thread_message_id, ',')   threads
            FROM chat_messages
            JOIN contacts ON chat_messages.contact_id = contacts.id AND chat_messages.tenant = contacts.tenant
            LEFT JOIN chat_message_reactions ON chat_message_reactions.root_message_id = chat_messages.message_id
            LEFT JOIN chat_message_threads ON chat_message_threads.root_message_id = chat_messages.message_id
            WHERE chat_messages.timestamp > (SELECT (timestamp) FROM chat_messages WHERE message_id = ?)
            AND chat_messages.tenant = ?
            GROUP BY chat_messages.message_id
            ORDER BY timestamp ASC",
        )
        .bind(message_id)
        .bind(self.tenant());

        let res = query.fetch(self.pool().await);

        let mapped = res.map(|e| {
            e.map(|row| Message {
                sender: ContactDisplay {
                    id: row.get("contact_id"),
                    display_name: row.get("display_name"),
                    alternative_name: row.try_get("alternative_name").ok(),
                    protocol: row.get("protocol"),
                    preview_profile_picture: row
                        .try_get::<Vec<u8>, &str>("low_res_profile_picture")
                        .ok()
                        .map(|e| e.into()),
                },
                message_id: row.get("message_id"),
                protocol_message_meta: serde_json::from_value(row.get("protocol_message_meta"))
                    .unwrap(),
                reactions: row
                    .get::<&str, &str>("reactions")
                    .split(',')
                    .map(|e| MessageId(e.parse().unwrap()))
                    .collect(),
                replies: row
                    .get::<&str, &str>("threads")
                    .split(',')
                    .map(|e| MessageId(e.parse().unwrap()))
                    .collect(),
                timestamp: row.get::<i64, &str>("timestamp") as u64,
                content: serde_json::from_value(row.get("message_content")).unwrap(),
            })
        });

        mapped
    }

    pub async fn get_previous_messages(
        &self,
        message_id: MessageId,
    ) -> Map<
        BoxStream<sqlx::Result<SqliteRow>>,
        impl FnMut(sqlx::Result<SqliteRow>) -> sqlx::Result<Message>,
    > {
        let query = sqlx::query(
            "SELECT message_id,
                message_content,
                timestamp,
                contact_id,
                display_name,
                alternative_name,
                low_res_profile_picture,
                protocol,
                protocol_message_meta,
                GROUP_CONCAT(reaction_message_id, ',') reactions,
                GROUP_CONCAT(thread_message_id, ',')   threads
            FROM chat_messages
            JOIN contacts ON chat_messages.contact_id = contacts.id AND chat_messages.tenant = contacts.tenant
            LEFT JOIN chat_message_reactions ON chat_message_reactions.root_message_id = chat_messages.message_id
            LEFT JOIN chat_message_threads ON chat_message_threads.root_message_id = chat_messages.message_id
            WHERE chat_messages.timestamp <= (SELECT (timestamp) FROM chat_messages WHERE message_id = ?)
            AND chat_messages.tenant = ?
            GROUP BY chat_messages.message_id
            ORDER BY timestamp DESC",
        )
        .bind(message_id)
        .bind(self.tenant());

        let res = query.fetch(self.pool().await);

        let mapped = res.map(|e| {
            e.map(|row| Message {
                sender: ContactDisplay {
                    id: row.get("contact_id"),
                    display_name: row.get("display_name"),
                    alternative_name: row.try_get("alternative_name").ok(),
                    protocol: row.get("protocol"),
                    preview_profile_picture: row
                        .try_get::<Vec<u8>, &str>("low_res_profile_picture")
                        .ok()
                        .map(|e| e.into()),
                },
                message_id: row.get("message_id"),
                protocol_message_meta: serde_json::from_value(row.get("protocol_message_meta"))
                    .unwrap(),
                reactions: row
                    .get::<&str, &str>("reactions")
                    .split(',')
                    .map(|e| MessageId(e.parse().unwrap()))
                    .collect(),
                replies: row
                    .get::<&str, &str>("threads")
                    .split(',')
                    .map(|e| MessageId(e.parse().unwrap()))
                    .collect(),
                timestamp: row.get::<i64, &str>("timestamp") as u64,
                content: serde_json::from_value(row.get("message_content")).unwrap(),
            })
        });

        mapped
    }

    pub async fn mycelink_message_id_to_message_id(
        &self,
        mycelink_id: &MycelinkChatMessageId,
    ) -> sqlx::Result<Option<MessageId>> {
        let query = sqlx::query(
            "SELECT message_id
                 FROM chat_messages
                 JOIN chat_ids on chat_ids.id = chat_messages.chat_id
                 WHERE protocol = ?
                    AND json_extract(protocol_message_meta, '$.id') = ?
                    AND chat_ids.tenant = ?;",
        )
        .bind(Protocol::Mycelink)
        .bind(mycelink_id.0.as_slice())
        .bind(self.tenant());

        query
            .fetch_optional(self.pool().await)
            .await
            .map(|e| e.map(|row| row.get("message_id")))
    }

    /// Stores a Message into the database
    /// The id field is ignored as the database generates a new message_id. The new id is then returned.
    pub async fn store_message(
        &self,
        contact_id: ContactId,
        message_content: &MessageType,
        protocol_message_meta: ProtocolMessageMeta,
        timestamp: u64,
        chat_id: ChatId,
    ) -> sqlx::Result<MessageId> {
        let query = sqlx::query("
            INSERT INTO chat_messages (chat_id, contact_id, protocol_message_meta, message_content, timestamp, tenant)
            VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(chat_id)
            .bind(contact_id)
            .bind(Json(protocol_message_meta))
            .bind(Json(message_content))
            .bind(timestamp as i64)
            .bind(self.tenant());

        query
            .execute(self.pool().await)
            .await
            .map(|e| MessageId(e.last_insert_rowid()))
    }
}
