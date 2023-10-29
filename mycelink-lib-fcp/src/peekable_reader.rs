use crate::decode_error::DecodeError;
use pin_project_lite::pin_project;
use std::cmp::min;
use std::collections::VecDeque;
use std::io::Write;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader, ReadBuf};

pin_project! {
    pub struct PeekableReader<T: AsyncRead> {
        #[pin]
        inner: T,
        buffer: VecDeque<u8>,
    }
}

impl<T: AsyncRead + Unpin> PeekableReader<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: VecDeque::new(),
        }
    }

    pub async fn peek_exact(&mut self, buf: &mut [u8]) -> Result<(), tokio::io::Error> {
        if buf.len() > self.buffer.len() {
            let mut inner_read = vec![0; buf.len() - self.buffer.len()];

            self.inner.read_exact(inner_read.as_mut_slice()).await?;

            self.buffer.write_all(inner_read.as_mut_slice())?;
        }

        let mut slices = self.buffer.as_slices();
        if self.buffer.len() > buf.len() {
            slices.0 = slices.0.split_at(min(slices.0.len(), buf.len())).0;
            slices.1 = slices
                .1
                .split_at(min(slices.1.len(), buf.len() - slices.1.len()))
                .0;
        }

        let destinations = buf.split_at_mut(slices.0.len());

        destinations.0.copy_from_slice(slices.0);
        destinations.1.copy_from_slice(slices.1);

        Ok(())
    }

    /// Consumes n bytes from the internal buffer
    ///
    /// # Panics
    /// Will panic if n is larger than the length of the internal buffer.
    /// You should only call this method if you previously used ['peek_exact'] with a buffer larger than n and there were no reads in between.
    /// You can also call ['available'] to check the maximum number of bytes that can be safely consumed.
    pub fn consume(&mut self, n: usize) {
        self.buffer.drain(0..n);
    }

    /// Returns the amount of bytes in the internal buffer
    pub fn available(&self) -> usize {
        self.buffer.len()
    }

    /// Finds a specific pattern within the internal buffer.
    /// Returns the index (inclusive) of the last byte of the first occurrence (after the start_search_at) of the pattern.
    ///
    /// An empty pattern always returns Some(0) unless the internal buffer is empty.
    /// None is returned if the pattern cannot be found.
    ///
    /// # Panics
    /// Panics if start search at is greater than the number of bytes within the internal buffer minus one.
    fn find_pattern(&self, pattern: &[u8], start_search_at: usize) -> Option<usize> {
        let mut correct_in_order = 0;
        for i in start_search_at..self.buffer.len() {
            if self.buffer[i] == pattern[correct_in_order] {
                correct_in_order += 1;

                if correct_in_order == pattern.len() {
                    return Some(i);
                }
            }
        }

        return None;
    }
}

impl<T: AsyncRead + Unpin> PeekableReader<BufReader<T>> {
    pub async fn peek_until(
        &mut self,
        buf: &mut Vec<u8>,
        pattern: &[u8],
    ) -> Result<(), tokio::io::Error> {
        if pattern.is_empty() {return Ok(())};

        if let Some(i) = self.find_pattern(pattern, 0) {
            buf.extend(self.buffer.iter().take(i+1))
            return Ok(())
        }


        self.inner.read_until(*pattern.last().unwrap(), buf).await;

        todo!()
    }
}

impl<T: AsyncRead> AsyncRead for PeekableReader<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if !self.buffer.is_empty() {
            let n = min(buf.remaining(), self.buffer.len());

            let slices = self.buffer.as_slices();

            if n <= slices.0.len() {
                buf.put_slice(slices.0.split_at(n).0);
            } else {
                buf.put_slice(slices.0);
                buf.put_slice(slices.1.split_at(n - slices.0.len()).0)
            }

            let me = self.project();
            me.buffer.drain(0..n);

            Poll::Ready(Ok(()))
        } else {
            let me = self.project();
            Poll::Ready(ready!(me.inner.poll_read(cx, buf)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::peekable_reader::PeekableReader;
    use tokio::io::AsyncReadExt;
    use tokio_test::io::Builder;

    #[tokio::test]
    async fn test_read_single() {
        let test_data = [1; 10];
        let mock = Builder::new().read(&test_data).build();

        let mut reader = PeekableReader::new(mock);

        let mut dest = [0; 10];
        reader.read_exact(&mut dest).await.unwrap();

        assert_eq!(dest, test_data);
    }

    #[tokio::test]
    async fn test_peek_single() {
        let test_data = [1; 10];
        let mock = Builder::new().read(&test_data).build();

        let mut reader = PeekableReader::new(mock);

        let mut dest = [0; 10];
        reader.peek_exact(&mut dest).await.unwrap();

        assert_eq!(dest, test_data);
    }

    #[tokio::test]
    async fn test_read_peek_and_read_same() {
        let test_data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mock = Builder::new().read(&test_data).build();

        let mut reader = PeekableReader::new(mock);

        let mut first_read_dest = [0; 3];
        reader.read_exact(&mut first_read_dest).await.unwrap();

        let mut peek_dest = [0; 5];
        reader.peek_exact(&mut peek_dest).await.unwrap();

        let mut second_read_dest = [0; 7];
        reader.read_exact(&mut second_read_dest).await.unwrap();

        assert_eq!(first_read_dest, test_data.split_at(3).0);
        assert_eq!(peek_dest, test_data.split_at(3).1.split_at(5).0);
        assert_eq!(second_read_dest, test_data.split_at(3).1);
    }

    #[tokio::test]
    async fn test_peek_over_multiple() {
        let test_data = [1; 10];
        let test_date_splits = test_data.split_at(3);
        let mock = Builder::new()
            .read(test_date_splits.0)
            .read(test_date_splits.1)
            .build();

        let mut reader = PeekableReader::new(mock);

        let mut dest = [0; 10];
        reader.peek_exact(&mut dest).await.unwrap();

        assert_eq!(dest, test_data);
    }
}
