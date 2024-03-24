use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::SubscribeUSK;
use crate::model::message_type_identifier::MessageType::Client;
use crate::model::priority_class::PriorityClass;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::model::uri::URI;

pub struct SubscribeUSKMessage {
    pub uri: URI,
    pub dont_poll: bool,
    pub identifier: UniqueIdentifier,
    pub priority_class: PriorityClass,
    // pub prio_progress_ u16,
    pub real_time: bool,
    pub sparse_poll: bool,
    pub ignore_usk_datehints: bool,
}

impl From<SubscribeUSKMessage> for Message {
    fn from(value: SubscribeUSKMessage) -> Self {
        let fields = vec![
            Field::new("Identifier".into(), (&value.identifier).into()),
            Field::new("URI".into(), (&value.uri).into()),
            Field::new("DontPoll".into(), value.dont_poll.to_string().into()),
            Field::new("SparsePoll".into(), value.sparse_poll.to_string().into()),
            Field::new("PriorityClass".into(), (&value.priority_class).into()),
            Field::new("RealTimeFlag".into(), value.real_time.to_string().into()),
            Field::new(
                "IgnoreUSKDatehints".into(),
                value.ignore_usk_datehints.to_string().into(),
            ),
        ];

        Self::new(Client(SubscribeUSK), fields.into(), None)
    }
}
