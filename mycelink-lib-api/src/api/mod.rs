use crate::db::db_connector::DBConnector;
use crate::model::account::Account;
use crate::model::chat::{Chat, ChatMetadata};
use crate::model::contact::{Contact, ContactIdentifier};
use crate::model::media::{Media, MediaId};
use crate::model::message::{Message, MessageId};

pub struct APIConnector<L: LoginStatus> {
    login_status: L,
    db_connector: DBConnector,
}

pub trait LoginStatus {}
type NotSignedIn = ();
type SignedIn = Account;
impl LoginStatus for NotSignedIn {}
impl LoginStatus for SignedIn {}

impl APIConnector<NotSignedIn> {
    pub fn new(db_connector: DBConnector) -> Self {
        Self {
            login_status: (),
            db_connector,
        }
    }

    pub fn open_account(&self, ssk_public_key: Box<str>) -> APIConnector<SignedIn> {
        todo!()
    }
}

impl<L: LoginStatus> APIConnector<L> {
    pub fn list_account_ssk_keys(&self) -> Box<[Box<str>]> {
        todo!();
    }

    pub fn create_account(&self) -> Box<str> {
        todo!()
    }
}

impl APIConnector<SignedIn> {
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

    pub fn get_messages_before(
        &self,
        limit: usize,
        guardian: Option<MessageId>,
        filter: Option<ChatMetadata>,
    ) -> Box<MessageId> {
        todo!()
    }

    pub fn get_media(&self, media_id: MediaId) -> Media {
        todo!()
    }
}
