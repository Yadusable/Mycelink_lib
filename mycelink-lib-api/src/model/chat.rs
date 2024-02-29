use crate::model::connection_details::PublicMycelinkConnectionDetails;

pub enum ChatMetadata {
    Mycelink { inner: MycelinkChatMetadata },
}

pub enum MycelinkChatMetadata {
    DirectChat {
        recipent: PublicMycelinkConnectionDetails,
    },
}
