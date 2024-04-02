use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::mycelink::mycelink_contact::MycelinkContact;
use crate::mycelink::protocol::mycelink_channel::MycelinkChannel;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChat {
    chat_type: MycelinkChatType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MycelinkChatType {
    DirectChat {
        channel: MycelinkChannel,
        contact: MycelinkContact,
    },
}

impl MycelinkChat {
    fn fetch(&self, db: &DBConnector<Tenant>, fcp: &FCPConnector) {}
}
