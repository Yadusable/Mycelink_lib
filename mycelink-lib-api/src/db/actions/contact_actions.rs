use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::model::connection_details::PublicConnectionDetails;
use crate::model::contact::ContactDisplay;
use crate::model::protocol_config::Protocol;
use crate::mycelink::mycelink_contact::MycelinkContact;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{SqliteArgumentValue, SqliteTypeInfo};
use sqlx::types::Json;
use sqlx::{Decode, Encode, Row, Sqlite, Type};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ContactId(i64);

impl Decode<'_, Sqlite> for ContactId {
    fn decode(value: <Sqlite as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let id = <i64 as Decode<Sqlite>>::decode(value)?;
        Ok(ContactId(id))
    }
}
impl Type<Sqlite> for ContactId {
    fn type_info() -> SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for ContactId {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        buf.push(SqliteArgumentValue::Int64(self.0));
        IsNull::No
    }
}

impl DBConnector<Tenant> {
    pub async fn get_contact_display(
        &self,
        contact_id: ContactId,
    ) -> sqlx::Result<Option<ContactDisplay>> {
        let query = sqlx::query(
            "SELECT (id, display_name, alternative_name, low_res_profile_picture, protocol)\
            FROM contacts\
            WHERE id = ?\
            AND tenant = ?",
        )
        .bind(contact_id)
        .bind(self.tenant());

        Ok(query
            .fetch_optional(self.pool().await)
            .await?
            .map(|row| ContactDisplay {
                id: row.get("id"),
                display_name: row.get("display_name"),
                alternative_name: row.get("alternative_name"),
                protocol: row.get("protocol"),
                preview_profile_picture: row
                    .try_get::<Vec<u8>, &str>("low_res_profile_picture")
                    .ok()
                    .map(|e| e.into()),
            }))
    }

    pub async fn list_contacts(&self) -> impl Stream<Item = sqlx::Result<ContactDisplay>> + '_ {
        let query = sqlx::query("SELECT id, display_name, alternative_name, low_res_profile_picture, protocol FROM contacts WHERE tenant = ?;").bind(self.tenant());

        query.fetch(self.pool().await).map(|e| {
            e.map(|row| ContactDisplay {
                id: row.get("id"),
                display_name: row.get("display_name"),
                alternative_name: row.try_get("alternative_name").ok(),
                protocol: row.get("protocol"),
                preview_profile_picture: row.try_get("low_res_profile_picture").ok(),
            })
        })
    }

    pub async fn mycelink_contact_id_to_contact_id(
        &self,
        contact: &MycelinkContact,
    ) -> sqlx::Result<Option<ContactId>> {
        let query =
            sqlx::query("SELECT id FROM contacts where connection_details = ? AND tenant = ?;")
                .bind(Json(contact))
                .bind(self.tenant());

        query
            .fetch_optional(self.pool().await)
            .await
            .map(|e| e.map(|row| row.get("id")))
    }

    pub async fn add_contact(
        &self,
        connection_details: PublicConnectionDetails,
        display_name: &str,
        profile_picture: Option<&[u8]>,
        low_res_profile_picture: Option<&[u8]>,
    ) -> sqlx::Result<ContactId> {
        let protocol = match connection_details {
            PublicConnectionDetails::Mycelink { .. } => Protocol::Mycelink,
        };

        let query = sqlx::query("INSERT INTO contacts (display_name, profile_picture, low_res_profile_picture, protocol, connection_details, tenant) VALUES (?,?,?,?,?,?);")
            .bind(display_name)
            .bind(profile_picture)
            .bind(low_res_profile_picture)
            .bind(protocol)
            .bind(Json(connection_details))
            .bind(self.tenant());

        let contact_id = query.execute(self.pool().await).await?.last_insert_rowid();
        Ok(ContactId(contact_id))
    }
}
