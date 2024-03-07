use mycelink_lib_fcp::messages::all_data::AllDataMessage;
use mycelink_lib_fcp::messages::client_get::ClientGetMessage;
use mycelink_lib_fcp::messages::client_hello::ClientHelloMessage;
use mycelink_lib_fcp::messages::client_put::ClientPutMessage;
use mycelink_lib_fcp::messages::generate_ssk::GenerateSSKMessage;
use mycelink_lib_fcp::messages::node_hello::NodeHelloMessage;
use mycelink_lib_fcp::messages::ssk_keypair::SSKKeypairMessage;
use mycelink_lib_fcp::messages::subscribe_usk::SubscribeUSKMessage;
use mycelink_lib_fcp::messages::subscribed_usk_update::SubscribedUSKUpdateMessage;
use mycelink_lib_fcp::model::fcp_version::FCPVersion;
use mycelink_lib_fcp::model::message::{FCPEncodable, Message};
use mycelink_lib_fcp::model::message_type_identifier::{MessageType, NodeMessageType};
use mycelink_lib_fcp::model::persistence::Persistence;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::return_type::ReturnType;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use mycelink_lib_fcp::model::upload_type::UploadType;
use mycelink_lib_fcp::model::uri::URI;
use mycelink_lib_fcp::peekable_reader::PeekableReader;
use std::time::UNIX_EPOCH;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

pub const FCP_PORT_1: u16 = 9481;
pub const FCP_PORT_2: u16 = 9482;

pub async fn prepare_connection(
    port: u16,
) -> (OwnedWriteHalf, PeekableReader<BufReader<OwnedReadHalf>>) {
    let tag = UNIX_EPOCH.elapsed().unwrap().as_millis();
    let mut stream = TcpStream::connect(format!("localhost:{port}"))
        .await
        .unwrap();

    let client_hello = ClientHelloMessage {
        name: format!("benches-{tag}").into(),
        version: FCPVersion::V2_0,
    };
    let encoded = client_hello.to_message().encode();

    stream.write_all(encoded.as_slice()).await.unwrap();

    let (rx, tx) = stream.into_split();

    let mut reader = PeekableReader::new(BufReader::new(rx));

    let _node_hello: NodeHelloMessage = Message::decode(&mut reader)
        .await
        .unwrap()
        .try_into()
        .unwrap();

    (tx, reader)
}

pub async fn generate_ssk(
    tx: &mut (impl AsyncWrite + Unpin),
    rx: &mut PeekableReader<impl AsyncRead + Unpin>,
) -> SSKKeypairMessage {
    let generate_ssk = GenerateSSKMessage {
        identifier: UniqueIdentifier::new("Generate SSK"),
    };
    let encoded = generate_ssk.to_message().encode();
    tx.write_all(encoded.as_slice()).await.unwrap();

    Message::decode(rx).await.unwrap().try_into().unwrap()
}

pub async fn receive_message(rx: &mut PeekableReader<impl AsyncRead + Unpin>) -> Message {
    let message = Message::decode(rx).await.unwrap();
    //println!("Received Message {:?}", message);
    message
}

pub async fn measure_put_time(put_message: ClientPutMessage, receive_uri: URI) {
    let mut conn_1 = prepare_connection(FCP_PORT_1).await;
    let mut conn_2 = prepare_connection(FCP_PORT_2).await;

    conn_1
        .0
        .write_all((&put_message).to_message().encode().as_slice())
        .await
        .unwrap();

    let get = ClientGetMessage {
        identifier: UniqueIdentifier::new("Measure Put"),
        uri: receive_uri,
        verbosity: Default::default(),
        return_type: ReturnType::Direct,
        max_size: None,
        max_temp_size: None,
        max_retries: 20,
        priority: PriorityClass::High,
        persistence: Persistence::Connection,
        ignore_data_store: true,
        data_store_only: false,
        real_time: true,
    };

    conn_2
        .0
        .write_all((&get).to_message().encode().as_slice())
        .await
        .unwrap();

    loop {
        let message;
        tokio::select! {
            e = receive_message(&mut conn_1.1) => message = e,
            e = receive_message(&mut conn_2.1) => message = e,
        }

        match message.message_type() {
            MessageType::Client(_) => {
                panic!()
            }
            MessageType::Node(message) => match message {
                NodeMessageType::DataFound => {}
                NodeMessageType::PutSuccessful => {}
                NodeMessageType::PutFailed => {
                    panic!("Received PutFailed")
                }
                NodeMessageType::GetFailed => {
                    panic!("Received GetFailed")
                }
                NodeMessageType::URIGenerated => {}
                NodeMessageType::AllData => {
                    break;
                }
                invalid_type => panic!("Received unexpected message Type {invalid_type}"),
            },
        }
    }
}

pub async fn measure_usk_update_time(put_message: ClientPutMessage, receive_uri: URI) {
    let mut conn_1 = prepare_connection(FCP_PORT_1).await;
    let mut conn_2 = prepare_connection(FCP_PORT_2).await;

    conn_1
        .0
        .write_all((&put_message).to_message().encode().as_slice())
        .await
        .unwrap();

    let subscribe = SubscribeUSKMessage {
        uri: receive_uri.clone(),
        dont_poll: false,
        identifier: UniqueIdentifier::new("Measure update usk"),
        priority_class: PriorityClass::High,
        real_time: false,
        sparse_poll: false,
        ignore_usk_datehints: false,
    };

    conn_2
        .0
        .write_all(subscribe.to_message().encode().as_slice())
        .await
        .unwrap();

    loop {
        let message;
        tokio::select! {
            e = receive_message(&mut conn_1.1) => message = e,
            e = receive_message(&mut conn_2.1) => message = e,
        }

        match message.message_type() {
            MessageType::Client(_) => {
                panic!()
            }
            MessageType::Node(message_type) => match message_type {
                NodeMessageType::AllData => {
                    let msg: AllDataMessage = message.try_into().unwrap();

                    if (UploadType::Direct { data: msg.data }) == put_message.upload_from {
                        break;
                    }
                }
                NodeMessageType::DataFound => {}
                NodeMessageType::SubscribedUSK => {}
                NodeMessageType::PutSuccessful => {}
                NodeMessageType::PutFailed => {
                    panic!("Received PutFailed")
                }
                NodeMessageType::GetFailed => {
                    panic!("Received GetFailed")
                }
                NodeMessageType::URIGenerated => {}
                NodeMessageType::SubscribedUSKUpdate => {
                    let update: SubscribedUSKUpdateMessage = message.try_into().unwrap();

                    let get = ClientGetMessage {
                        identifier: UniqueIdentifier::new("get update usk"),
                        uri: update.uri,
                        verbosity: Default::default(),
                        return_type: ReturnType::Direct,
                        max_size: None,
                        max_temp_size: None,
                        max_retries: 0,
                        priority: PriorityClass::High,
                        persistence: Persistence::Connection,
                        ignore_data_store: false,
                        data_store_only: false,
                        real_time: true,
                    };

                    conn_2
                        .0
                        .write_all((&get).to_message().encode().as_slice())
                        .await
                        .unwrap();
                }
                NodeMessageType::SubscribedUSKSendingToNetwork => {}
                NodeMessageType::SubscribedUSKRoundFinished => {}
                invalid_type => panic!("Received unexpected message Type {invalid_type}"),
            },
        }
    }

    loop {
        let message;
        tokio::select! {
            e = receive_message(&mut conn_1.1) => message = e,
            e = receive_message(&mut conn_2.1) => message = e,
        }

        match message.message_type() {
            MessageType::Client(_) => {
                panic!()
            }
            MessageType::Node(message_type) => match message_type {
                NodeMessageType::SubscribedUSK => {}
                NodeMessageType::SubscribedUSKUpdate => {}
                NodeMessageType::PutSuccessful => {}
                NodeMessageType::PutFailed => {
                    panic!("Received PutFailed")
                }
                NodeMessageType::GetFailed => {
                    panic!("Received GetFailed")
                }
                NodeMessageType::URIGenerated => {}
                NodeMessageType::SubscribedUSKSendingToNetwork => {}
                NodeMessageType::SubscribedUSKRoundFinished => {}
                NodeMessageType::DataFound => {}
                NodeMessageType::AllData => {
                    assert_eq!(
                        UploadType::Direct {
                            data: message.payload().unwrap().data
                        },
                        put_message.upload_from
                    );
                    break;
                }
                invalid_type => panic!("Received unexpected message Type {invalid_type}"),
            },
        }
    }
}
