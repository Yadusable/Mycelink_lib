pub mod create_account;

use crate::db::db_connector::{DBConnector, NoTenant, TenantState};
use crate::model::account::Account;
use crate::model::chat::ChatMetadata;
use crate::model::contact::{Contact, ContactIdentifier};
use crate::model::media::{Media, MediaId};
use crate::model::message::Message;
use crate::model::tenant::Tenant;
use mycelink_lib_fcp::fcp_connector::FCPConnector;

pub struct APIConnector<L: LoginStatus, T: TenantState> {
    login_status: L,
    db_connector: DBConnector<T>,
    fcp_connector: FCPConnector,
}

pub trait LoginStatus {}

type NotSignedIn = ();
type SignedIn = Account;

impl LoginStatus for NotSignedIn {}

impl LoginStatus for SignedIn {}

impl APIConnector<NotSignedIn, Tenant> {
    pub fn open_account(&self, ssk_public_key: Box<str>) -> APIConnector<SignedIn, Tenant> {
        todo!()
    }
}

impl APIConnector<NotSignedIn, NoTenant> {
    pub fn enter_tenant() -> APIConnector<NotSignedIn, Tenant> {
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

    pub fn list_account_public_ssk_keys(&self) -> Box<[Box<str>]> {
        todo!();
    }

    pub fn list_account_ssk_keys(&self) -> Box<[Box<str>]> {
        todo!();
    }

    pub fn health_check(&self) -> Result<(), ()> {
        Ok(())
    }
}

impl APIConnector<SignedIn, Tenant> {
    pub fn add_contact(&self, contact: &Contact) {
        todo!()
    }

    pub fn friend_request(&self, contact_identifier: &ContactIdentifier) {
        todo!()
    }

    pub fn list_friend_request(&self) -> Box<[Box<str>]> {
        todo!()
    }

    pub fn send_message(&self, message: Message, chat: &ChatMetadata) {
        todo!()
    }

    pub fn get_media(&self, media_id: MediaId) -> Media {
        todo!()
    }
}
