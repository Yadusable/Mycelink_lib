use mycelink_lib_fcp::fcp_parser_legacy::FCPParserLegacy;
use mycelink_lib_fcp::messages::client_hello::{ClientHelloMessage, EXPECTED_VERSION};
use mycelink_lib_fcp::messages::node_hello::NodeHelloMessage;
use mycelink_lib_fcp::messages::ClientMessage::ClientHello;
use mycelink_lib_fcp::messages::{FCPDecodable, FCPEncodable};
use mycelink_lib_fcp::model::fcp_version::FCPVersion;
use mycelink_lib_fcp::model::message::Message;
use mycelink_lib_fcp::peekable_reader::PeekableReader;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::test]
async fn test_client_hello() {
    let mut stream = TcpStream::connect("localhost:9481").await.unwrap();

    let client_hello = ClientHello(ClientHelloMessage {
        version: EXPECTED_VERSION,
        name: "Integration_test".into(),
    });

    let encoded = Into::<Message>::into(client_hello).encode();

    stream.write_all(encoded.as_bytes()).await.unwrap();

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
