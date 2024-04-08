use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::model::contact::ContactDisplay;
use futures::{Stream, StreamExt};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{SqliteArgumentValue, SqliteTypeInfo};
use sqlx::{Decode, Encode, Row, Sqlite, Type};

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
}
