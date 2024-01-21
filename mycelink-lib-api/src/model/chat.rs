use crate::model::contact::ContactIdentifier;

pub enum ChatMetadata {
    DirectChat { recipient: ContactIdentifier },
}
