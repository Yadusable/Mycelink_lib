use mycelink_lib_fcp::messages::client_hello::ClientHelloMessage;
use mycelink_lib_fcp::messages::node_hello::NodeHelloMessage;
use mycelink_lib_fcp::model::fcp_version::FCPVersion;
use mycelink_lib_fcp::model::message::{FCPEncodable, Message};
use mycelink_lib_fcp::peekable_reader::PeekableReader;
use tokio::io::{AsyncWriteExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

pub async fn prepare_connection() -> (
    WriteHalf<TcpStream>,
    PeekableReader<BufReader<ReadHalf<TcpStream>>>,
) {
    let mut stream = TcpStream::connect("localhost:9481").await.unwrap();

    let client_hello = ClientHelloMessage {
        name: "benches".to_string().into(),
        version: FCPVersion::V2_0,
    };
    let encoded = client_hello.to_message().encode();

    stream.write_all(encoded.as_slice()).unwrap();

    let (rx, tx) = stream.split();

    let mut reader = PeekableReader::new(BufReader::new(rx));

    let _node_hello: NodeHelloMessage = Message::decode(&mut reader)
        .await
        .unwrap()
        .try_into()
        .unwrap();

    (tx, reader)
}
