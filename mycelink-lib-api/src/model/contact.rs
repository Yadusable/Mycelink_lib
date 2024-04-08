use crate::db::actions::contact_actions::ContactId;
use crate::model::protocol_config::Protocol;

pub struct ContactDisplay {
    pub id: ContactId,
    pub display_name: Box<str>,
    pub alternative_name: Option<Box<str>>,
    pub protocol: Protocol,
    /// A low res version of the profile picture
    pub preview_profile_picture: Option<Box<[u8]>>,
}

pub enum ContactSetupDetails {
    Mycelink { account_request_key: Box<str> },
}
