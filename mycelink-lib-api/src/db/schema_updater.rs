use crate::db::db_connector::DatabaseBackend;
use sqlx::{Pool, Transaction};

pub async fn update_to_newest_version(
    current_version: u32,
    pool: &Pool<DatabaseBackend>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    update_to_v1(current_version, &mut tx).await?;

    tx.commit().await?;
    Ok(())
}

async fn update_to_v1(
    current_version: u32,
    tx: &mut Transaction<'_, DatabaseBackend>,
) -> Result<(), sqlx::Error> {
    match current_version {
        1.. => Ok(()),
        0 => {
            log::info!("Updating db schema to v1");
            let query = sqlx::query(include_str!("db_schema_v1.sql"));
            query.execute(&mut **tx).await?;

            let query = sqlx::query("INSERT INTO database_metadata (schema_version) VALUES (1)");
            query.execute(&mut **tx).await?;
            Ok(())
        }
    }
}
