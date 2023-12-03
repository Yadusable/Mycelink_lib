use crate::model::content_type::ContentType;
use crate::model::fields::Field;
use crate::model::message::{Message, MessagePayload};
use crate::model::message_type_identifier::ClientMessageType::ClientPut;
use crate::model::message_type_identifier::MessageType::Client;
use crate::model::persistence::Persistence;
use crate::model::priority_class::PriorityClass;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::model::upload_type::UploadType;
use crate::model::uri::URI;
use crate::model::verbosity::Verbosity;

pub struct ClientPutMessage {
    pub uri: URI,
    pub content_type: Option<ContentType>,
    pub identifier: UniqueIdentifier,
    pub verbosity: Verbosity,
    pub max_retries: i32,
    pub priority: PriorityClass,
    pub get_only_chk: bool,
    pub dont_compress: bool,
    pub persistence: Persistence,
    pub target_filename: Option<Box<str>>,
    pub upload_from: UploadType,
    pub is_binary_blob: bool,
    pub real_time: bool,
}

impl From<&ClientPutMessage> for Message {
    fn from(value: &ClientPutMessage) -> Self {
        let mut fields = vec![
            Field::new("Identifier".into(), (&value.identifier).into()),
            Field::new("URI".into(), (&value.uri).into()),
            Field::new("Verbosity".into(), (&value.verbosity).into()),
            Field::new("MaxRetries".into(), value.max_retries.to_string().into()),
            Field::new("PriorityClass".into(), (&value.priority).into()),
            Field::new("GetCHKOnly".into(), value.get_only_chk.to_string().into()),
            Field::new(
                "DontCompress".into(),
                value.dont_compress.to_string().into(),
            ),
            Field::new("Persistence".into(), (&value.persistence).into()),
            Field::new("UploadFrom".into(), (&value.upload_from).into()),
            Field::new("BinaryBlob".into(), value.is_binary_blob.to_string().into()),
            Field::new("RealTimeFlag".into(), value.real_time.to_string().into()),
        ];

        if let Some(content_type) = &value.content_type {
            fields.push(Field::new(
                "Metadata.ContentType".into(),
                content_type.into(),
            ));
        }
        if let Some(filename) = &value.target_filename {
            fields.push(Field::new("TargetFilename".into(), filename.clone()))
        }
        if let UploadType::Disk { path } = &value.upload_from {
            fields.push(Field::new("Filename".into(), path.to_string_lossy().into()))
        }
        if let UploadType::Redirect { target } = &value.upload_from {
            fields.push(Field::new("TargetURI".into(), target.into()))
        }

        let payload = match &value.upload_from {
            UploadType::Direct { data } => Some(MessagePayload {
                data: data.clone(),
                data_len_identifier: "DataLength".into(),
            }),
            _ => None,
        };

        Message::new(Client(ClientPut), fields.into(), payload)
    }
}
