use std::path::Path;

pub struct DBConnector {
    connection: (),
}

impl DBConnector {
    pub fn new(db_path: &Path) -> Result<Self, ()> {
        todo!()
    }

    fn initial_setup(&mut self) {
        todo!()
    }

    fn current_schema_version(&self) -> u32 {
        todo!()
    }

    pub fn has_account(&self, public_key: &str) {
        todo!()
    }
}
