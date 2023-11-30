use std::collections::VecDeque;
use std::ops::Sub;
use std::str::from_utf8;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};

pub struct PeekableReader<T: AsyncRead> {
    inner: BufReader<T>,
    peekable_lines: VecDeque<Arc<str>>,
}

impl<T: AsyncRead + Unpin> PeekableReader<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: BufReader::new(inner),
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

        let line = self.inner_read_line().await?;
        match line {
            None => Ok(None),
            Some(line) => {
                self.peekable_lines.push_back(line.clone());
                Ok(Some(line))
            }
        }
    }

    pub async fn read_line(&mut self) -> Result<Option<Arc<str>>, tokio::io::Error> {
        let cached = self.peekable_lines.pop_front();

        if cached.is_some() {
            return Ok(cached);
        }

        self.inner_read_line().await
    }

    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), tokio::io::Error> {
        assert!(
            self.peekable_lines.is_empty(),
            "Cannot read while lines are still cached"
        );

        self.inner.read_exact(buf).await?;

        Ok(())
    }

    async fn inner_read_line(&mut self) -> Result<Option<Arc<str>>, tokio::io::Error> {
        let mut buf = Vec::new();
        let read_len = self.inner.read_until(b'\n', &mut buf).await?;

        if read_len == 0 {
            return Ok(None);
        }

        let line = from_utf8(buf.as_slice()).unwrap(); //todo for now just panic when encountering non utf8 data. Handle it gracefully in the future?

        Ok(Some(line.split_at(line.len() - 1).0.into()))
    }

    pub async fn read_contentful_line(&mut self) -> Result<Option<Arc<str>>, tokio::io::Error> {
        let mut res = self.read_line().await?;
        while res.as_ref().map(|e| e.trim_end() == "").unwrap_or(false) {
            res = self.read_line().await?;
        }

        Ok(res)
    }

    pub fn advance_to_peeker_stats(&mut self, stats: PeekerStats) {
        self.peekable_lines.drain(..stats.current_line);
    }
}

pub struct Peeker<'a, T: AsyncRead> {
    reader: &'a mut PeekableReader<T>,
    next_line: usize,
}

impl<'a, T: AsyncRead + Unpin> Peeker<'a, T> {
    pub fn new(reader: &'a mut PeekableReader<T>) -> Self {
        Self {
            reader,
            next_line: 0,
        }
    }

    pub async fn next_line(&mut self) -> Result<Option<Arc<str>>, tokio::io::Error> {
        self.next_line += 1;
        self.reader.get_peeked_line(self.next_line - 1).await
    }

    pub async fn next_contentful_line(&mut self) -> Result<Option<Arc<str>>, tokio::io::Error> {
        let mut res = self.next_line().await?;
        while res.as_ref().map(|e| e.trim_end() == "").unwrap_or(false) {
            res = self.next_line().await?;
        }

        Ok(res)
    }

    pub async fn has_next_line(&mut self) -> Result<bool, tokio::io::Error> {
        self.reader
            .get_peeked_line(self.next_line)
            .await
            .map(|e| e.is_some())
    }

    pub async fn current_line(&mut self) -> Result<Option<Arc<str>>, tokio::io::Error> {
        if self.next_line == 0 {
            panic!("Peeker hasn't been used yet and therefore has no current line")
        }

        self.reader.get_peeked_line(self.next_line - 1).await
    }
}

pub struct PeekerStats {
    current_line: usize,
}

impl<'a, T: AsyncRead> From<Peeker<'a, T>> for PeekerStats {
    fn from(value: Peeker<T>) -> Self {
        Self {
            current_line: value.next_line,
        }
    }
}

impl Sub<usize> for PeekerStats {
    type Output = PeekerStats;

    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            current_line: self.current_line - rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::peekable_reader::{PeekableReader, Peeker};
    use std::ops::Deref;
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

    #[tokio::test]
    async fn test_next_contentful_line() {
        let mock = Builder::new().read(b"Line One\n\n\nLine Two\n").build();

        let mut reader = PeekableReader::new(mock);

        let mut peeker1 = Peeker::new(&mut reader);

        assert_eq!(
            peeker1
                .next_contentful_line()
                .await
                .unwrap()
                .unwrap()
                .deref(),
            "Line One"
        );
        assert_eq!(
            peeker1
                .next_contentful_line()
                .await
                .unwrap()
                .unwrap()
                .deref(),
            "Line Two"
        );

        assert_eq!(
            reader
                .read_contentful_line()
                .await
                .unwrap()
                .unwrap()
                .deref(),
            "Line One"
        );
        assert_eq!(
            reader
                .read_contentful_line()
                .await
                .unwrap()
                .unwrap()
                .deref(),
            "Line Two"
        );
    }
}
