use crate::decode_error::DecodeError;
use crate::decode_error::DecodeError::{ExpectedDifferentMessage, UnknownMessageIdentifier};
use crate::model::fields::{Field, Fields};
use crate::model::message_identifier::{
    MessageIdentifier, NodeMessageIdentifier, NODE_MESSAGE_IDENTIFIERS,
};
use crate::peekable_reader::PeekableReader;
use tokio::io::{AsyncRead, BufReader};

const END_MESSAGE_LIT: &str = "EndMessage\n";
const DATA_LIT: &str = "Data\n";

pub struct FCPParser<'a, T: AsyncRead + Unpin> {
    reader: &'a mut PeekableReader<BufReader<T>>,
}

impl<'a, T: AsyncRead + Unpin> FCPParser<'a, T> {
    pub fn new(reader: &'a mut PeekableReader<BufReader<T>>) -> Self {
        Self { reader }
    }

    pub async fn expect_node_identifier(
        &mut self,
        expected: NodeMessageIdentifier,
    ) -> Result<(), DecodeError> {
        let read = self.peek_node_identifier().await?;

        if read == expected {
            self.reader.consume(read.name().len());
            Ok(())
        } else {
            Err(ExpectedDifferentMessage {
                expected,
                got: read,
            })
        }
    }

    pub async fn peek_node_identifier(&mut self) -> Result<NodeMessageIdentifier, DecodeError> {
        let mut buf = Vec::new();
        self.reader.peek_until(&mut buf, &[b'\n']).await?;

        match NODE_MESSAGE_IDENTIFIERS
            .iter()
            .find(|e| e.name().as_bytes() == buf.as_slice())
        {
            None => Err(UnknownMessageIdentifier {
                got: String::from_utf8_lossy(buf.as_slice()).into(),
            }),
            Some(found) => Ok(*found),
        }
    }

    pub async fn parse_fields(&mut self) -> Result<Fields, DecodeError> {
        let mut peek_buf = Vec::new();
        let mut results = Vec::new();
        while {
            peek_buf.clear();
            self.reader.peek_until(&mut peek_buf, b"\n").await?;
            peek_buf.as_slice() != "EndMessage\n".as_bytes()
                && peek_buf.as_slice() != "Data\n".as_bytes()
        } {
            let line = String::from_utf8_lossy(peek_buf.as_slice());
            let line = line.split_at(line.len().saturating_sub(1)).0;

            match line.split_once('=') {
                None => {
                    self.discard_message()
                        .await
                        .expect("Failed to recover from invalid Fields while parsing message.");
                    return Err(DecodeError::ParseError(format!("Expected separator '=' in '{line}' while parsing fields. Discarding message.").into()));
                }
                Some((key, value)) => results.push(Field::new(key.into(), value.into())),
            };

            self.reader.consume(peek_buf.len());
        }

        self.reader.consume(peek_buf.len());
        Ok(results.into())
    }

    async fn discard_message(&mut self) -> Result<(), DecodeError> {
        let mut peek_buf = Vec::new();
        while {
            peek_buf.clear();
            self.reader
                .peek_until(&mut peek_buf, "\n".as_bytes())
                .await?;
            peek_buf.as_slice() != END_MESSAGE_LIT.as_bytes()
                && peek_buf.as_slice() != DATA_LIT.as_bytes()
        } {
            self.reader.consume(peek_buf.len())
        }

        if peek_buf.as_slice() == DATA_LIT.as_bytes() {
            todo!() // TODO somehow consume remaining data
        }

        Ok(())
    }
}
