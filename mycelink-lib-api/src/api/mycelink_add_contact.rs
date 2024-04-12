use crate::api::APIConnector;
use crate::db::actions::tenant_actions::Tenant;
use crate::fcp_tools::fcp_get::{fcp_get_inline, FcpGetError};
use crate::model::connection_details::{PublicConnectionDetails, PublicMycelinkConnectionDetails};
use crate::model::contact::ContactDisplay;
use crate::model::protocol_config::Protocol;
use mycelink_lib_fcp::decode_error::DecodeError;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use std::ops::Deref;

impl APIConnector<Tenant> {
    pub async fn add_mycelink_contact(
        &self,
        account_request_key: Box<str>,
    ) -> Result<ContactDisplay, AddContactError> {
        let details = fcp_get_inline(
            account_request_key.deref().try_into()?,
            self.fcp_connector.deref(),
            "add_contact",
            PriorityClass::High,
        )
        .await?;
        let public_details: PublicMycelinkConnectionDetails =
            ciborium::from_reader(details.data.as_ref())?;

        let display_name = public_details.display_name().clone();
        let contact_id = self
            .db_connector
            .add_contact(
                PublicConnectionDetails::Mycelink(public_details),
                display_name.deref(),
                None,
                None,
            )
            .await?;

        Ok(ContactDisplay {
            id: contact_id,
            display_name,
            alternative_name: None,
            protocol: Protocol::Mycelink,
            preview_profile_picture: None,
        })
    }
}

#[derive(Debug)]
pub enum AddContactError {
    Sqlx(sqlx::Error),
    Ciborium(ciborium::de::Error<std::io::Error>),
    FcpGet(FcpGetError),
    Uri(DecodeError),
}

impl From<sqlx::Error> for AddContactError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<ciborium::de::Error<std::io::Error>> for AddContactError {
    fn from(value: ciborium::de::Error<std::io::Error>) -> Self {
        Self::Ciborium(value)
    }
}

impl From<FcpGetError> for AddContactError {
    fn from(value: FcpGetError) -> Self {
        Self::FcpGet(value)
    }
}

impl From<DecodeError> for AddContactError {
    fn from(value: DecodeError) -> Self {
        Self::Uri(value)
    }
}
