use mycelink_lib_fcp::messages::client_hello::{ClientHelloMessage, EXPECTED_VERSION};
use mycelink_lib_fcp::messages::node_hello::NodeHelloMessage;
use mycelink_lib_fcp::model::fcp_version::FCPVersion;
use mycelink_lib_fcp::model::message::ClientMessage::ClientHello;
use mycelink_lib_fcp::model::message::{FCPEncodable, Message};
use mycelink_lib_fcp::peekable_reader::PeekableReader;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::test]
async fn integration_client_hello() {
    let mut stream = TcpStream::connect("localhost:9481").await.unwrap();

    let client_hello = ClientHello(ClientHelloMessage {
        version: EXPECTED_VERSION,
        name: "Integration_test_client_hello".into(),
    });

    let encoded = client_hello.to_message().encode();

    stream.write_all(encoded.as_slice()).await.unwrap();

    let (rx, _tx) = stream.split();

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
    )
}
