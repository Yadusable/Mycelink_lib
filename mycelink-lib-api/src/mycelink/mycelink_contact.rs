use crate::model::connection_details::PublicMycelinkConnectionDetails;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkContact {
    display_name: Box<str>,
    connection_details: PublicMycelinkConnectionDetails,
}
