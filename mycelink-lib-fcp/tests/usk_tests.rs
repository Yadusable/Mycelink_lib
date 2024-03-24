use mycelink_lib_fcp::messages::client_hello::{ClientHelloMessage, EXPECTED_VERSION};
use mycelink_lib_fcp::messages::client_put::ClientPutMessage;
use mycelink_lib_fcp::messages::generate_ssk::GenerateSSKMessage;
use mycelink_lib_fcp::messages::node_hello::NodeHelloMessage;
use mycelink_lib_fcp::messages::put_successful::PutSuccessfulMessage;
use mycelink_lib_fcp::messages::ssk_keypair::SSKKeypairMessage;
use mycelink_lib_fcp::messages::subscribe_usk::SubscribeUSKMessage;
use mycelink_lib_fcp::messages::uri_generated::UriGeneratedMessage;
use mycelink_lib_fcp::model::fcp_version::FCPVersion;
use mycelink_lib_fcp::model::message::{FCPEncodable, Message};
use mycelink_lib_fcp::model::persistence::Persistence;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use mycelink_lib_fcp::model::upload_type::UploadType;
use mycelink_lib_fcp::model::verbosity::Verbosity;
use mycelink_lib_fcp::peekable_reader::PeekableReader;
use rand::RngCore;
use std::ops::Deref;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::test]
async fn test_usk_behaviour() {
    let mut stream = TcpStream::connect("localhost:9481").await.unwrap();

    // Handshake
    // #############################################################################################

    let client_hello = ClientHelloMessage {
        version: EXPECTED_VERSION,
        name: "Integration_usk_tests".into(),
    };

    let encoded = client_hello.to_message().encode();

    stream.write_all(encoded.as_slice()).await.unwrap();

    let (rx, mut tx) = stream.split();

    let mut peekable_reader = PeekableReader::new(rx);
    let message = Message::decode(&mut peekable_reader).await.unwrap();
    println!("{message:?}");

    let node_hello: NodeHelloMessage = message.try_into().unwrap();

    assert_eq!(
        node_hello,
        NodeHelloMessage {
            fcp_version: FCPVersion::V2_0,
            node: "Fred".into(),
            connection_identifier: node_hello.connection_identifier.clone(),
        }
    );

    // Generate SSK
    // #############################################################################################
    let generate_ssk_identifier = UniqueIdentifier::new("Generate Test");
    let generate_ssk = GenerateSSKMessage {
        identifier: generate_ssk_identifier,
    };
    let encoded = generate_ssk.to_message().encode();

    tx.write_all(encoded.as_slice()).await.unwrap();

    let message = Message::decode(&mut peekable_reader).await.unwrap();
    println!("{message:?}");
    let ssk_keypair: SSKKeypairMessage = message.try_into().unwrap();

    // Put 1
    // #############################################################################################
    let client_put_identifier = UniqueIdentifier::new("Put Test");
    let mut payload_bytes = [0; 128];
    rand::thread_rng().fill_bytes(&mut payload_bytes);

    let mut insert_usk_uri = ssk_keypair.insert_uri.deref().replace("SSK@", "USK@");
    insert_usk_uri.push_str("test/0/");
    let client_put = ClientPutMessage {
        uri: insert_usk_uri.as_str().try_into().unwrap(),
        content_type: None,
        identifier: client_put_identifier.clone(),
        verbosity: Verbosity {
            simple_progress: false,
            sending_to_network: false,
            compatibility_mode: false,
            expected_hashes: false,
            expected_mime: false,
            expected_data_length: false,
        },
        max_retries: 0,
        priority: PriorityClass::Medium,
        get_only_chk: false,
        dont_compress: true,
        persistence: Persistence::Connection,
        target_filename: None,
        upload_from: UploadType::Direct {
            data: payload_bytes.to_vec(),
        },
        is_binary_blob: false,
        real_time: true,
    };

    let encoded = client_put.to_message().encode();

    tx.write_all(encoded.as_slice()).await.unwrap();

    let message = Message::decode(&mut peekable_reader).await.unwrap();
    println!("{message:?}");
    let generated_uri_message: UriGeneratedMessage = message.try_into().unwrap();

    let message = Message::decode(&mut peekable_reader).await.unwrap();
    println!("{message:?}");
    let put_sucessful: PutSuccessfulMessage = message.try_into().unwrap();

    // Put 2
    // #############################################################################################
    let client_put_identifier = UniqueIdentifier::new("Put Test");
    let mut payload_bytes = [0; 128];
    rand::thread_rng().fill_bytes(&mut payload_bytes);

    let mut insert_usk_uri = ssk_keypair.insert_uri.deref().replace("SSK@", "USK@");
    insert_usk_uri.push_str("test/0");
    let client_put = ClientPutMessage {
        uri: insert_usk_uri.as_str().try_into().unwrap(),
        content_type: None,
        identifier: client_put_identifier.clone(),
        verbosity: Verbosity {
            simple_progress: false,
            sending_to_network: false,
            compatibility_mode: false,
            expected_hashes: false,
            expected_mime: false,
            expected_data_length: false,
        },
        max_retries: 0,
        priority: PriorityClass::Medium,
        get_only_chk: false,
        dont_compress: true,
        persistence: Persistence::Connection,
        target_filename: None,
        upload_from: UploadType::Direct {
            data: payload_bytes.to_vec(),
        },
        is_binary_blob: false,
        real_time: true,
    };

    let encoded = client_put.to_message().encode();

    tx.write_all(encoded.as_slice()).await.unwrap();

    let message = Message::decode(&mut peekable_reader).await.unwrap();
    println!("{message:?}");
    let generated_uri_message: UriGeneratedMessage = message.try_into().unwrap();

    let message = Message::decode(&mut peekable_reader).await.unwrap();
    println!("{message:?}");
    let put_sucessful: PutSuccessfulMessage = message.try_into().unwrap();

    // SubscribeUSK
    // #############################################################################################
    let identifier = UniqueIdentifier::new("USK Tests");
    let mut request_usk_uri = ssk_keypair.request_uri.replace("SSK@", "USK@");
    request_usk_uri.push_str("test/1/");

    let subscribe_message = SubscribeUSKMessage {
        uri: request_usk_uri.as_str().try_into().unwrap(),
        dont_poll: false,
        identifier,
        priority_class: PriorityClass::High,
        real_time: false,
        sparse_poll: false,
        ignore_usk_datehints: false,
    };

    let encoded = subscribe_message.to_message().encode();

    tx.write_all(encoded.as_slice()).await.unwrap();

    loop {
        let message = Message::decode(&mut peekable_reader).await.unwrap();
        println!("{message:?}")
    }
}
