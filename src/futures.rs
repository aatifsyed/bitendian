use super::*;
use futures_io::AsyncRead;
use pin_project::pin_project;
use std::{
    future::Future,
    io,
    marker::PhantomData,
    pin::Pin,
    task::{ready, Context, Poll},
};

fn assert_future<T, F: Future<Output = T>>(f: F) -> F {
    f
}

#[pin_project]
pub struct ReadEndian<const N: usize, R, T> {
    #[pin]
    reader: R,
    buffer: [u8; N],
    count: usize,
    endian: Endian,
    _out: PhantomData<T>,
}

impl<const N: usize, R, T> Future for ReadEndian<N, R, T>
where
    R: AsyncRead,
    T: ByteOrder<N>,
{
    type Output = io::Result<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();
        loop {
            *this.count += ready!(this
                .reader
                .as_mut()
                .poll_read(cx, &mut this.buffer[*this.count..]))?;
            if *this.count >= N {
                return Poll::Ready(Ok(T::from_bytes_endian(self.buffer, self.endian)));
            }
        }
    }
}

impl<const N: usize, R, T> ReadEndian<N, R, T> {
    fn new(reader: R, endian: Endian) -> Self {
        Self {
            reader,
            buffer: [0u8; N],
            count: 0,
            endian,
            _out: PhantomData,
        }
    }
}

pub trait AsyncReadExt<const N: usize>: AsyncRead + Unpin {
    fn read_endian<T: ByteOrder<N>>(&mut self, endian: Endian) -> ReadEndian<N, &mut Self, T> {
        assert_future::<io::Result<T>, _>(ReadEndian::new(self, endian))
    }
    fn read_be<T: ByteOrder<N>>(&mut self) -> ReadEndian<N, &mut Self, T> {
        self.read_endian(Endian::Big)
    }
    fn read_le<T: ByteOrder<N>>(&mut self) -> ReadEndian<N, &mut Self, T> {
        self.read_endian(Endian::Little)
    }
    fn read_ne<T: ByteOrder<N>>(&mut self) -> ReadEndian<N, &mut Self, T> {
        self.read_endian(Endian::Native)
    }
}
impl<const N: usize, R> AsyncReadExt<N> for R where R: AsyncRead + Unpin {}
