use mycelink_lib_fcp::messages::client_hello::{ClientHelloMessage, EXPECTED_VERSION};
use mycelink_lib_fcp::messages::client_put::ClientPutMessage;
use mycelink_lib_fcp::messages::node_hello::NodeHelloMessage;
use mycelink_lib_fcp::messages::put_successful::PutSuccessfulMessage;
use mycelink_lib_fcp::model::fcp_version::FCPVersion;
use mycelink_lib_fcp::model::message::ClientMessage::{ClientHello, ClientPut};
use mycelink_lib_fcp::model::message::{FCPEncodable, Message};
use mycelink_lib_fcp::model::persistence::Persistence;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use mycelink_lib_fcp::model::upload_type::UploadType;
use mycelink_lib_fcp::model::verbosity::Verbosity;
use mycelink_lib_fcp::peekable_reader::PeekableReader;
use rand::RngCore;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::test]
async fn integration_put_get() {
    let mut stream = TcpStream::connect("localhost:9481").await.unwrap();

    // Handshake
    // #############################################################################################

    let client_hello = ClientHello(ClientHelloMessage {
        version: EXPECTED_VERSION,
        name: "Integration_test_put_get".into(),
    });

    let encoded = client_hello.to_message().encode();

    stream.write_all(encoded.as_slice()).await.unwrap();

    let (rx, mut tx) = stream.split();

    let mut peekable_reader = PeekableReader::new(rx);
    let message = Message::decode(&mut peekable_reader).await.unwrap();

    let node_hello: NodeHelloMessage = message.try_into().unwrap();

    assert_eq!(
        node_hello,
        NodeHelloMessage {
            fcp_version: FCPVersion::V2_0,
            node: "Fred".into(),
            connection_identifier: node_hello.connection_identifier.clone(),
        }
    );

    // Put
    // #############################################################################################
    let client_put_identifier = UniqueIdentifier::new("Put Test");
    let mut payload_bytes = [0; 128];
    rand::thread_rng().fill_bytes(&mut payload_bytes);
    let client_put = ClientPut(ClientPutMessage {
        uri: "CHK@".try_into().unwrap(),
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
    });

    let encoded = client_put.to_message().encode();

    tx.write_all(encoded.as_slice()).await.unwrap();

    let _message = Message::decode(&mut peekable_reader).await.unwrap();
    //TODO check for generated uri message

    let message = Message::decode(&mut peekable_reader).await.unwrap();
    let put_sucessful: PutSuccessfulMessage = message.try_into().unwrap();

    assert_eq!(put_sucessful.identifier, client_put_identifier)
}
