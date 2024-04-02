pub mod mycelink_create_account;

use crate::db::actions::contact_actions::ContactId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::{DBConnector, NoTenant, TenantState};
use crate::model::chat::Chat;
use crate::model::config::Config;
use crate::model::connection_details::PublicConnectionDetails;
use crate::model::contact::ContactDisplay;
use crate::model::media::{Media, MediaId};
use crate::mycelink::mycelink_account::MycelinkAccount;
use futures::stream::BoxStream;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use std::path::Path;

pub struct APIConnector<L: LoginStatus, T: TenantState> {
    login_status: L,
    db_connector: DBConnector<T>,
    fcp_connector: FCPConnector,
}

pub trait LoginStatus {}

type NotSignedIn = ();
type SignedIn = MycelinkAccount;

impl LoginStatus for NotSignedIn {}

impl LoginStatus for SignedIn {}

impl APIConnector<NotSignedIn, Tenant> {
    pub fn open_mycelink_account(&self) -> APIConnector<SignedIn, Tenant> {
        todo!()
    }
}

impl APIConnector<NotSignedIn, NoTenant> {
    pub fn enter_tenant() -> APIConnector<NotSignedIn, Tenant> {
        todo!()
    }
}

impl APIConnector<(), ()> {
    pub async fn init(config: Config) -> Result<APIConnector<(), ()>, ()> {
        todo!()
    }

    pub async fn enter_demo(self) -> Result<APIConnector<SignedIn, Tenant>, ()> {
        todo!()
    }
}

impl<L: LoginStatus, T: TenantState> APIConnector<L, T> {
    pub fn new(
        db_connector: DBConnector<T>,
        fcp_connector: FCPConnector,
    ) -> APIConnector<NotSignedIn, T> {
        APIConnector {
            login_status: (),
            db_connector,
            fcp_connector,
        }
    }

    pub fn health_check(&self) -> Result<(), ()> {
        Ok(())
    }
}

impl APIConnector<SignedIn, Tenant> {
    pub async fn list_contacts(&self) -> BoxStream<ContactDisplay> {
        todo!()
    }

    pub async fn create_direct_chat(&self, contact_id: ContactId) -> Result<Chat, ()> {
        todo!()
    }
    pub async fn list_chats(&self) -> BoxStream<Chat> {
        todo!()
    }

    pub async fn add_contact(
        &self,
        connection_details: PublicConnectionDetails,
    ) -> Result<ContactDisplay, ()> {
        todo!()
    }

    pub fn current_account_request_key(&self) -> Box<str> {
        todo!()
    }
}
