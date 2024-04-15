use crate::model::connection_details::PublicMycelinkConnectionDetails;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkContact {
    display_name: Box<str>,
    connection_details: PublicMycelinkConnectionDetails,
}

impl MycelinkContact {
    pub fn new(
        display_name: Box<str>,
        connection_details: PublicMycelinkConnectionDetails,
    ) -> Self {
        Self {
            display_name,
            connection_details,
        }
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    pub fn connection_details(&self) -> &PublicMycelinkConnectionDetails {
        &self.connection_details
    }
}
