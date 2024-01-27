use crate::api::{APIConnector, LoginStatus};
use std::error::Error;
use std::fmt::{Display, Formatter};

impl<L: LoginStatus> APIConnector<L> {
    pub async fn create_account(&self) -> Box<str> {
        
    }
}

#[derive(Debug)]
pub enum CreateAccountErrors {
    AlreadyExists { public_ssk_key: Box<str> },
}

impl Error for CreateAccountErrors {}
impl Display for CreateAccountErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAccountErrors::AlreadyExists { public_ssk_key } => {
                write!(f, "Account with public_ssk_key '{public_ssk_key}' already exists")
            }
        }
    }
}
