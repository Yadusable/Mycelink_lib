use crate::decode_error::DecodeError;
use crate::decode_error::DecodeError::{ExpectedDifferentMessageType, UnknownMessageType};
use crate::model::fields::{Field, Fields};
use crate::model::message_type_identifier::{NodeMessageType, NODE_MESSAGE_TYPES};
use crate::peekable_reader_legacy::PeekableReaderLegacy;
use tokio::io::{AsyncRead, BufReader};

const END_MESSAGE_LIT: &str = "EndMessage\n";
const DATA_LIT: &str = "Data\n";

#[deprecated]
pub struct FCPParserLegacy<'a, T: AsyncRead + Unpin> {
    reader: &'a mut PeekableReaderLegacy<BufReader<T>>,
}

impl<'a, T: AsyncRead + Unpin> FCPParserLegacy<'a, T> {
    pub fn new(reader: &'a mut PeekableReaderLegacy<BufReader<T>>) -> Self {
        Self { reader }
    }

    pub async fn expect_node_identifier(
        &mut self,
        expected: NodeMessageType,
    ) -> Result<(), DecodeError> {
        let read = self.peek_node_identifier().await?;

        if read == expected {
            self.reader.consume(read.name().len() + 1);
            Ok(())
        } else {
            Err(ExpectedDifferentMessageType {
                expected,
                got: read,
            })
        }
    }

    pub async fn peek_node_identifier(&mut self) -> Result<NodeMessageType, DecodeError> {
        let mut buf = Vec::new();
        self.reader.peek_until(&mut buf, &[b'\n']).await?;

        match NODE_MESSAGE_TYPES
            .iter()
            .find(|e| e.name().as_bytes() == buf.as_slice().split_at(buf.len() - 1).0)
        {
            None => Err(UnknownMessageType {
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
