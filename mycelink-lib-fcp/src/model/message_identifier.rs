use crate::model::message_identifier::MessageIdentifier::{ClientHello, NodeHello};
use std::fmt::{Display, Formatter};

pub const MESSAGE_IDENTIFIERS: &[MessageIdentifier] = &[ClientHello, NodeHello];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageIdentifier {
    ClientHello,
    NodeHello,
}

impl MessageIdentifier {
    /// Gets the str representing the MessageIdentifier (e.g. "ClientHello")
    pub fn name(&self) -> &'static str {
        match self {
            ClientHello => "ClientHello\n",
            NodeHello => "NodeHello\n",
        }
    }
}

impl Display for MessageIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
