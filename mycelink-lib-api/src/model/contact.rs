use crate::db::actions::contact_actions::ContactId;

pub struct ContactDisplay {
    pub id: ContactId,
    pub display_name: Box<str>,
    pub alternative_name: Option<Box<str>>,
    pub protocol: Box<str>,
    /// A low res version of the profile picture
    pub preview_profile_picture: Box<[u8]>,
}

pub enum ContactSetupDetails {
    Mycelink { account_request_key: Box<str> },
}
