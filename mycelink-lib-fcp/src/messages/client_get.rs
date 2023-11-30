use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::ClientGet;
use crate::model::message_type_identifier::MessageType::Client;
use crate::model::persistence::Persistence;
use crate::model::priority_class::PriorityClass;
use crate::model::return_type::ReturnType;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::model::uri::URI;
use crate::model::verbosity::Verbosity;

pub struct ClientGetMessage {
    pub identifier: UniqueIdentifier,
    pub uri: URI,
    pub verbosity: Verbosity,
    pub return_type: ReturnType,
    pub max_size: Option<usize>,
    pub max_temp_size: Option<usize>,
    pub max_retries: i32,
    pub priority: PriorityClass,
    pub persistence: Persistence,
    /// Always fetches from external source
    pub ignore_data_store: bool,
    /// Only checks in local datastore
    pub data_store_only: bool,
    pub real_time: bool,
}

impl From<&ClientGetMessage> for Message {
    fn from(value: &ClientGetMessage) -> Self {
        let mut fields = vec![
            Field::new("Identifier".into(), (&value.identifier).into()),
            Field::new("uri".into(), (&value.uri).into()),
            Field::new("Verbosity".into(), (&value.verbosity).into()),
            Field::new("ReturnType".into(), (&value.return_type).into()),
            Field::new("MaxRetries".into(), value.max_retries.to_string().into()),
            Field::new("Priority".into(), (&value.priority).into()),
            Field::new("Persistence".into(), (&value.persistence).into()),
            Field::new(
                "IgnoreDS".into(),
                value.ignore_data_store.to_string().into(),
            ),
            Field::new("DSonly".into(), value.data_store_only.to_string().into()),
            Field::new("RealTimeFlag".into(), value.real_time.to_string().into()),
        ];

        if let ReturnType::Disk { path } = &value.return_type {
            fields.push(Field::new("Filename".into(), path.to_string_lossy().into()));
        }
        if let Some(max_size) = value.max_size {
            fields.push(Field::new("MaxSize".into(), max_size.to_string().into()));
        }
        if let Some(max_temp_size) = value.max_temp_size {
            fields.push(Field::new(
                "MaxTempSize".into(),
                max_temp_size.to_string().into(),
            ));
        }

        Message::new(Client(ClientGet), fields.into(), None)
    }
}
