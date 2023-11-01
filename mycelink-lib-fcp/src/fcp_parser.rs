use crate::decode_error::DecodeError;
use crate::model::message_identifier::MessageIdentifier;
use crate::peekable_reader::PeekableReader;
use std::fmt::format;
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

    pub async fn expect_identifier(
        &mut self,
        expected: MessageIdentifier,
    ) -> Result<(), DecodeError> {
        let expected = expected.name();
        let mut buf = vec![0; expected.len()];
        self.reader.peek_exact(buf.as_mut_slice()).await?;

        let got = String::from_utf8_lossy(buf.as_slice());

        if !got.starts_with(expected) {
            return Err(DecodeError::ExpectedDifferentMessage {
                expected,
                got: got.into(),
            });
        }

        self.reader.consume(buf.len());

        Ok(())
    }

    pub async fn parse_fields(&mut self) -> Result<Vec<(Box<str>, Box<str>)>, DecodeError> {
        let mut peek_buf = Vec::new();
        let mut results = Vec::new();
        while {
            peek_buf.clear();
            self.reader
                .peek_until(&mut peek_buf, "\n".as_bytes())
                .await?;
            peek_buf.as_slice() != "EndMessage\n".as_bytes()
                && peek_buf.as_slice() != "Data\n".as_bytes()
        } {
            let line = String::from_utf8_lossy(peek_buf.as_slice());

            match line.split_once('=') {
                None => {
                    self.discard_message()
                        .await
                        .expect("Failed to recover from invalid Fields while parsing message.");
                    return Err(DecodeError::ParseError(format!("Expected separator '=' in '{line}' while parsing fields. Discarding message.").into()));
                }
                Some((key, value)) => results.push((key.into(), value.into())),
            };

            self.reader.consume(peek_buf.len());
        }

        self.reader.consume(peek_buf.len());
        Ok(results)
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
