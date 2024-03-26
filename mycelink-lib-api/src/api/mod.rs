pub mod mycelink_create_account;

use crate::db::db_connector::{DBConnector, NoTenant, TenantState};
use crate::model::contact::Contact;
use crate::model::media::{Media, MediaId};
use crate::model::tenant::Tenant;
use crate::mycelink::mycelink_account::MycelinkAccount;
use mycelink_lib_fcp::fcp_connector::FCPConnector;

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
    pub fn add_mycelink_contact(
        &self,
        account_info_request_key: &str,
        display_name: impl Into<Box<str>>,
    ) {
        todo!()
    }

    pub fn mycelink_friend_request(&self, contact: Box<dyn Contact>) {
        todo!()
    }

    pub fn list_friend_request(&self) -> Box<[Box<str>]> {
        todo!()
    }

    pub fn get_media(&self, media_id: MediaId) -> Media {
        todo!()
    }
}
