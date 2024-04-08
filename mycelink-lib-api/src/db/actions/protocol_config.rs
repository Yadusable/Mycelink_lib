use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::model::protocol_config::{Protocol, ProtocolConfig};
use futures::{Stream, StreamExt};
use sqlx::Row;

impl DBConnector<Tenant> {
    pub async fn get_protocol_configs(
        &self,
    ) -> impl Stream<Item = sqlx::Result<ProtocolConfig>> + '_ {
        let query = sqlx::query("SELECT config FROM protocol_config_per_tenant WHERE tenant = ?;")
            .bind(self.tenant());

        query
            .fetch(self.pool().await)
            .map(|e| e.map(|row| serde_json::from_value(row.get("config")).unwrap()))
    }

    pub async fn get_protocol_config(
        &self,
        protocol: Protocol,
    ) -> sqlx::Result<Option<ProtocolConfig>> {
        let query = sqlx::query(
            "SELECT config FROM protocol_config_per_tenant WHERE protocol = ? AND tenant = ?",
        )
        .bind(protocol)
        .bind(self.tenant());

        query
            .fetch_optional(self.pool().await)
            .await
            .map(|e| e.map(|row| serde_json::from_value(row.get("protocol")).unwrap()))
    }
}
