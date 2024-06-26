use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MediaId {
    HyphanetResource { uri: Box<str> },
}

pub enum Media {}
