use std::cmp::min;
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};

pub struct PeekableReader<T: AsyncRead> {
    inner: T,
    buffer: VecDeque<u8>,
}

impl <T: AsyncRead> PeekableReader<T> {
    pub async fn peek_exact(&mut self, buf: &mut [u8]) -> Result<(), tokio::io::Error>{
        if buf.len() > self.buffer.len() {
            let mut inner_buf = vec![0; buf.len() - self.buffer.len()];

            self.inner.read_exact(inner_buf.as_mut_slice()).await?
        }

        todo!()
    }
}

impl <T: AsyncRead> AsyncRead for PeekableReader<T> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        if self.buffer.is_empty() {
            let n = min(buf.remaining(), self.buffer.len());

            let slices = self.buffer.as_slices();

            if n <= slices.0.len() {
                buf.put_slice(slices.0.split_at(n).0);
            } else {
                buf.put_slice(slices.0);
                buf.put_slice(slices.1.split_at(n - slices.0.len()).0)
            }

            self.buffer.drain(0..n);

            Poll::Ready(Ok(()))
        } else {
            self.inner.poll_read(cx, buf)
        }
    }
}