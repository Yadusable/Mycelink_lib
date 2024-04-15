use crate::api::APIConnector;
use crate::db::actions::chat_actions::ChatId;
use crate::db::actions::contact_actions::ContactId;
use crate::db::actions::tenant_actions::Tenant;
use crate::fcp_tools::fcp_get::{fcp_get_inline, FcpGetError};
use crate::model::connection_details::{PublicConnectionDetails, PublicMycelinkConnectionDetails};
use crate::model::contact::ContactDisplay;
use crate::model::protocol_config::Protocol;
use crate::mycelink::mycelink_chat::{MycelinkChat, OpenChatError};
use crate::mycelink::mycelink_contact::MycelinkContact;
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
                &PublicConnectionDetails::Mycelink(public_details),
                display_name.deref(),
                None,
                None,
            )
            .await?;

        self.create_direct_mycelink_chat_for_contact(contact_id)
            .await?;

        Ok(ContactDisplay {
            id: contact_id,
            display_name,
            alternative_name: None,
            protocol: Protocol::Mycelink,
            preview_profile_picture: None,
        })
    }

    async fn create_direct_mycelink_chat_for_contact(
        &self,
        contact_id: ContactId,
    ) -> Result<ChatId, OpenChatError> {
        let connection_details = self
            .db_connector
            .get_contact_connection_details(contact_id)
            .await?
            .ok_or(OpenChatError::ContactDoesntExist)?;

        if let PublicConnectionDetails::Mycelink(connection_details) = connection_details {
            let display = self
                .db_connector
                .get_contact_display(contact_id)
                .await
                .unwrap()
                .unwrap();

            let contact = MycelinkContact::new(display.display_name, connection_details);
            let account = self.db_connector.get_mycelink_account().await?.unwrap();

            let chat = MycelinkChat::new_direct_chat(&account, contact, self.fcp_connector.deref())
                .await?;
            let display_name: Box<str> = chat.display_name().into();
            Ok(self
                .db_connector
                .create_chat(display_name.as_ref(), chat.into())
                .await?)
        } else {
            Err(OpenChatError::ContactIsNotMycelink)
        }
    }
}

#[derive(Debug)]
pub enum AddContactError {
    Sqlx(sqlx::Error),
    Ciborium(ciborium::de::Error<std::io::Error>),
    FcpGet(FcpGetError),
    Uri(DecodeError),
    CreateChatError(OpenChatError),
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

impl From<OpenChatError> for AddContactError {
    fn from(value: OpenChatError) -> Self {
        Self::CreateChatError(value)
    }
}
