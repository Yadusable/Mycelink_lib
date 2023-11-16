use std::collections::VecDeque;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader, Lines};

pub struct PeekableReader<T: AsyncRead> {
    inner: Lines<BufReader<T>>,
    peekable_lines: VecDeque<Arc<str>>,
}

impl<T: AsyncRead + Unpin> PeekableReader<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: BufReader::new(inner).lines(),
            peekable_lines: VecDeque::new(),
        }
    }

    pub async fn get_peeked_line(
        &mut self,
        index: usize,
    ) -> Result<Option<Arc<str>>, tokio::io::Error> {
        let cached = self.peekable_lines.get(index);

        if cached.is_some() {
            return Ok(cached.cloned());
        }

        let line = self.inner.next_line().await?;

        if let Some(line) = line {
            let line: Arc<str> = line.into();
            self.peekable_lines.push_back(line.clone());
            return Ok(Some(line));
        }

        Ok(None)
    }

    pub async fn read_line(&mut self) -> Result<Option<Arc<str>>, tokio::io::Error> {
        let cached = self.peekable_lines.pop_front();

        if cached.is_some() {
            return Ok(cached);
        }

        self.inner.next_line().await.map(|e| e.map(|e| e.into()))
    }
}

pub struct Peeker<'a, T: AsyncRead> {
    reader: &'a mut PeekableReader<T>,
    current_line: usize,
}

impl<'a, T: AsyncRead + Unpin> Peeker<'a, T> {
    pub fn new(reader: &'a mut PeekableReader<T>) -> Self {
        Self {
            reader,
            current_line: 0,
        }
    }

    pub async fn next_line(&mut self) -> Result<Option<Arc<str>>, tokio::io::Error> {
        self.current_line += 1;
        self.reader.get_peeked_line(self.current_line - 1).await
    }

    pub async fn has_next_line(&mut self) -> Result<bool, tokio::io::Error> {
        self.reader
            .get_peeked_line(self.current_line)
            .await
            .map(|e| e.is_some())
    }
}

#[cfg(test)]
mod tests {
    use crate::peekable_reader::{PeekableReader, Peeker};
    use tokio_test::io::Builder;

    #[tokio::test]
    async fn test_multiple_peeker() {
        let mock = Builder::new().read(b"Line One\nLine Two\n").build();

        let mut reader = PeekableReader::new(mock);

        let mut peeker1 = Peeker::new(&mut reader);
        assert_eq!(&*peeker1.next_line().await.unwrap().unwrap(), "Line One");
        assert_eq!(&*peeker1.next_line().await.unwrap().unwrap(), "Line Two");

        let mut peeker2 = Peeker::new(&mut reader);
        assert_eq!(&*peeker2.next_line().await.unwrap().unwrap(), "Line One");
        assert_eq!(&*peeker2.next_line().await.unwrap().unwrap(), "Line Two");
    }

    #[tokio::test]
    async fn test_peek_and_read() {
        let mock = Builder::new().read(b"Line One\nLine Two\n").build();

        let mut reader = PeekableReader::new(mock);

        let mut peeker1 = Peeker::new(&mut reader);
        assert_eq!(&*peeker1.next_line().await.unwrap().unwrap(), "Line One");
        assert_eq!(&*peeker1.next_line().await.unwrap().unwrap(), "Line Two");

        assert_eq!(&*reader.read_line().await.unwrap().unwrap(), "Line One");
    }

    #[tokio::test]
    async fn test_read_and_peek() {
        let mock = Builder::new().read(b"Line One\nLine Two\n").build();

        let mut reader = PeekableReader::new(mock);

        assert_eq!(&*reader.read_line().await.unwrap().unwrap(), "Line One");

        let mut peeker1 = Peeker::new(&mut reader);
        assert_eq!(&*peeker1.next_line().await.unwrap().unwrap(), "Line Two");
    }

    #[tokio::test]
    async fn test_peek_read_peek() {
        let mock = Builder::new().read(b"Line One\nLine Two\n").build();

        let mut reader = PeekableReader::new(mock);

        let mut peeker1 = Peeker::new(&mut reader);
        assert_eq!(&*peeker1.next_line().await.unwrap().unwrap(), "Line One");

        assert_eq!(&*reader.read_line().await.unwrap().unwrap(), "Line One");

        let mut peeker2 = Peeker::new(&mut reader);
        assert_eq!(&*peeker2.next_line().await.unwrap().unwrap(), "Line Two");
    }
}
