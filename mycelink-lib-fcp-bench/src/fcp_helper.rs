use mycelink_lib_fcp::messages::client_hello::ClientHelloMessage;
use mycelink_lib_fcp::messages::generate_ssk::GenerateSSKMessage;
use mycelink_lib_fcp::messages::node_hello::NodeHelloMessage;
use mycelink_lib_fcp::messages::ssk_keypair::SSKKeypairMessage;
use mycelink_lib_fcp::model::fcp_version::FCPVersion;
use mycelink_lib_fcp::model::message::{FCPEncodable, Message};
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use mycelink_lib_fcp::peekable_reader::PeekableReader;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

pub async fn prepare_connection() -> (OwnedWriteHalf, PeekableReader<BufReader<OwnedReadHalf>>) {
    let mut stream = TcpStream::connect("localhost:9481").await.unwrap();

    let client_hello = ClientHelloMessage {
        name: "benches".to_string().into(),
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
    println!("Received Message {:?}", message);
    message
}
