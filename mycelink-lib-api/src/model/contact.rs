use crate::model::connection_details::PublicConnectionDetails;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Contact {
    #[serde(flatten)]
    internal_id: ContactId,
    display_name: Box<str>,
    public_connection_details: PublicConnectionDetails,
}

impl Contact {
    pub fn new(
        internal_id: ContactId,
        display_name: impl Into<Box<str>>,
        public_connection_details: PublicConnectionDetails,
    ) -> Self {
        Self {
            internal_id,
            display_name: display_name.into(),
            public_connection_details,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContactId {
    contact_id: u32,
}
