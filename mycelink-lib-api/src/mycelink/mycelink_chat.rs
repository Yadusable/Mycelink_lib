use crate::db::db_connector::DBConnector;
use crate::model::chat::Chat;
use crate::model::message::MessageId;
use crate::model::message_types::MessageType;
use crate::model::tenant::Tenant;
use crate::mycelink::mycelink_contact::MycelinkContact;
use crate::mycelink::protocol::mycelink_channel::MycelinkChannel;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use mycelink_lib_fcp::model::message::Message;
use crate::model::contact::Contact;

pub struct MycelinkChat {
    chat_type: MycelinkChatType,
}

pub enum MycelinkChatType {
    DirectChat {
        channel: MycelinkChannel,
        contact: MycelinkContact,
    },
}

impl MycelinkChat {
    fn fetch(&self, db: &DBConnector<Tenant>, fcp: &FCPConnector) {}
}

impl Chat for MycelinkChat {
    fn send_message(&mut self, message_type: MessageType) -> BoxFuture<Result<(), Box<str>>> {
        match &mut self.chat_type {
            MycelinkChatType::DirectChat { channel, .. } => {
                todo!()
            }
        }
    }

    fn open_message_streams_at(&self, message_id: MessageId) -> BoxStream<Message> {
        todo!()
    }

    fn open_message_streams_newest(&self) -> BoxStream<Message> {
        todo!()
    }

    fn display_name(&self) -> &str {
        match &self.chat_type {
            MycelinkChatType::DirectChat { contact, ..} => contact.display_name()
        }
    }

    fn alternative_name(&self) -> Option<&str> {
        None
    }
}
