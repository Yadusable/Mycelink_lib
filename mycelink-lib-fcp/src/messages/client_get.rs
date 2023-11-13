use crate::messages::FCPEncodable;
use crate::model::message_identifier::{ClientMessageIdentifier, MessageIdentifier};
use crate::model::persistence::Persistence;
use crate::model::priority_class::PriorityClass;
use crate::model::return_type::ReturnType;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::model::uri::URI;
use crate::model::verbosity::Verbosity;
use async_trait::async_trait;

const IDENTIFIER: ClientMessageIdentifier = ClientMessageIdentifier::ClientGet;

pub struct ClientGetMessage {
    identifier: UniqueIdentifier,
    uri: URI,
    verbosity: Verbosity,
    return_type: ReturnType,
    max_size: Option<usize>,
    max_temp_size: Option<usize>,
    max_retries: i32,
    priority: PriorityClass,
    persistence: Persistence,
    /// Always fetches from external source
    ignore_data_store: bool,
    /// Only checks in local datastore
    data_store_only: bool,
    real_time: bool,
}

#[async_trait]
impl FCPEncodable for ClientGetMessage {
    fn encode(&self) -> String {
        let mut builder = String::new();

        builder.push_str(IDENTIFIER.name());
        builder.push_str("Identifier=");
        builder.push_str(&Into::<Box<str>>::into(&self.identifier));
        builder.push_str("\nURI=");
        builder.push_str(&Into::<Box<str>>::into(&self.uri));
        builder.push_str("\nVerbosity=");
        builder.push_str(self.verbosity.as_bitmask().to_string().as_str());
        builder.push_str("\nReturnType=");
        builder.push_str((&self.return_type).into());
        if let ReturnType::Disk { path } = &self.return_type {
            builder.push_str("\nFilename=");
            builder.push_str(&path.to_string_lossy());
        }
        if let Some(max_size) = self.max_size {
            builder.push_str("\nMaxSize=");
            builder.push_str(max_size.to_string().as_str());
        }
        if let Some(max_temp_size) = self.max_temp_size {
            builder.push_str("\nMaxTempSize=");
            builder.push_str(max_temp_size.to_string().as_str());
        }
        builder.push_str("\nMaxRetries=");
        builder.push_str(self.max_retries.to_string().as_str());
        builder.push_str("\nPriority=");
        builder.push_str(&Into::<Box<str>>::into(&self.priority));
        builder.push_str("\nPersistence=");
        builder.push_str((&self.persistence).into());
        builder.push_str("\nIgnoreDS=");
        builder.push_str(self.ignore_data_store.to_string().as_str());
        builder.push_str("\nDSonly=");
        builder.push_str(self.data_store_only.to_string().as_str());
        builder.push_str("\nRealTimeFlag=");
        builder.push_str(self.real_time.to_string().as_str());

        builder.push_str("\nEndMessage\n");

        builder
    }
}
