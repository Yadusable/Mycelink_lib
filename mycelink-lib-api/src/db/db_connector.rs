use sqlx::{migrate::MigrateDatabase, Sqlite, SqliteConnection, Error};

use std::path::Path;

pub struct DBConnector {
    connection: SqliteConnection,
}

impl DBConnector {
    pub async fn new(db_path: &Path) -> Result<Self, sqlx::Error> {

        const DB_URL: &str = "sqlite://sqlite.db";
            if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
                println!("Creating database {}", DB_URL);
                match Sqlite::create_database(DB_URL).await {
                    Ok(_) => println!("Create db success"),
                    Err(error) => panic!("error: {}", error),
                }
            } else {
                println!("Database already exists");
            }

        Ok(Self { connection })
    }

    fn initial_setup(&mut self) {
        todo!()
    }

    fn current_schema_version(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn run_something() {

    }
}
