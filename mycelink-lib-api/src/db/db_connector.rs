use rusqlite::Connection;
use std::path::Path;

pub struct DBConnector {
    connection: Connection,
}

impl DBConnector {
    pub fn new(db_path: &Path) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(db_path)?;

        Ok(Self { connection })
    }

    fn initial_setup(&mut self) {
        todo!()
    }

    fn current_schema_version(&self) -> u32 {
        todo!()
    }
}
