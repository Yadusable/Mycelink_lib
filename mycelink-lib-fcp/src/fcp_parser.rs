use crate::decode_error::DecodeError;
use crate::model::MessageIdentifier::MessageIdentifier;
use crate::peekable_reader::PeekableReader;
use tokio::io::AsyncRead;

pub struct FCPParser<'a, T: AsyncRead + Unpin> {
    reader: &'a mut PeekableReader<T>,
}

impl<'a, T: AsyncRead + Unpin> FCPParser<'a, T> {
    pub fn new(reader: &'a mut PeekableReader<T>) -> Self {
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

    pub async
}
