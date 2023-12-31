//! Extension methods for asynchronous IO with [`tokio`](https://docs.rs/tokio/1/tokio/).
//!
//! ```
//! use bitendian::tokio::{AsyncReadExt as _, AsyncWriteExt as _};
//!
//! # async fn doit() -> std::io::Result<()> {
//! let mut buf = vec![];
//! buf.write_be(1u16).await?;
//! let swapped = buf.as_slice().read_le().await?;
//! assert_eq!(256u16, swapped);
//! # Ok(())
//! # }
//! # futures::executor::block_on(doit()).unwrap()
//! ```

use crate::{BitEndian, Endian};
use pin_project::pin_project;
use std::{
    future::Future,
    io,
    marker::PhantomData,
    pin::Pin,
    task::{ready, Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// Future for [`AsyncReadExt`], see that trait for more.
#[pin_project]
pub struct ReadEndian<const N: usize, R, T> {
    #[pin]
    reader: R,
    buffer: [u8; N],
    progress: usize,
    endian: Endian,
    _out: PhantomData<T>,
}

impl<const N: usize, R, T> Future for ReadEndian<N, R, T>
where
    R: AsyncRead,
    T: BitEndian<N>,
{
    type Output = io::Result<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();
        loop {
            let mut buf = ReadBuf::new(&mut this.buffer[*this.progress..]);
            ready!(this.reader.as_mut().poll_read(cx, &mut buf))?;
            let progress = buf.filled().len();
            if progress == 0 {
                return Poll::Ready(Err(io::Error::from(io::ErrorKind::UnexpectedEof)));
            }
            *this.progress += progress;
            if *this.progress >= N {
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
            progress: 0,
            endian,
            _out: PhantomData,
        }
    }
}

/// Extends [`tokio::io::AsyncRead`](https://docs.rs/tokio/1/tokio/io/trait.AsyncRead.html)
/// with methods for reading in an endian-dependant way.
///
/// See [module docs](mod@self) for usage examples.
pub trait AsyncReadExt<const N: usize>: AsyncRead + Unpin {
    /// Read according to a run-time endianness.
    fn read_endian<T: BitEndian<N>>(&mut self, endian: Endian) -> ReadEndian<N, &mut Self, T> {
        assert_future::<io::Result<T>, _>(ReadEndian::new(self, endian))
    }
    /// Read with [`Endian::Big`].
    fn read_be<T: BitEndian<N>>(&mut self) -> ReadEndian<N, &mut Self, T> {
        self.read_endian(Endian::Big)
    }
    /// Read with [`Endian::Little`].
    fn read_le<T: BitEndian<N>>(&mut self) -> ReadEndian<N, &mut Self, T> {
        self.read_endian(Endian::Little)
    }
    /// Read with [`Endian::Native`].
    fn read_ne<T: BitEndian<N>>(&mut self) -> ReadEndian<N, &mut Self, T> {
        self.read_endian(Endian::Native)
    }
}
impl<const N: usize, R> AsyncReadExt<N> for R where R: AsyncRead + Unpin {}

/// Future for [`AsyncWriteExt`], see that trait for more.
#[pin_project]
pub struct WriteArray<const N: usize, W> {
    #[pin]
    writer: W,
    buffer: [u8; N],
    progress: usize,
}

impl<const N: usize, W> Future for WriteArray<N, W>
where
    W: AsyncWrite,
{
    type Output = io::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();
        loop {
            *this.progress += ready!(this
                .writer
                .as_mut()
                .poll_write(cx, &this.buffer[*this.progress..]))?;
            if *this.progress >= N {
                return Poll::Ready(Ok(()));
            }
        }
    }
}

impl<const N: usize, W> WriteArray<N, W> {
    fn new(writer: W, it: impl BitEndian<N>, endian: Endian) -> Self {
        Self {
            writer,
            buffer: it.to_bytes_endian(endian),
            progress: 0,
        }
    }
}

/// Extends [`tokio::io::AsyncWrite`](https://docs.rs/tokio/1/tokio/io/trait.AsyncWrite.html)
/// with methods for writing in an endian-dependent way.
///
/// See [module docs](mod@self) for usage examples.
pub trait AsyncWriteExt<const N: usize>: AsyncWrite + Unpin {
    /// Write according to a run-time endianness.
    fn write_endian<T: BitEndian<N>>(&mut self, it: T, endian: Endian) -> WriteArray<N, &mut Self> {
        assert_future::<io::Result<()>, _>(WriteArray::new(self, it, endian))
    }
    /// Write with [`Endian::Big`].
    fn write_be<T: BitEndian<N>>(&mut self, it: T) -> WriteArray<N, &mut Self> {
        self.write_endian(it, Endian::Big)
    }
    /// Write with [`Endian::Little`].
    fn write_le<T: BitEndian<N>>(&mut self, it: T) -> WriteArray<N, &mut Self> {
        self.write_endian(it, Endian::Little)
    }
    /// Write with [`Endian::Native`].
    fn write_ne<T: BitEndian<N>>(&mut self, it: T) -> WriteArray<N, &mut Self> {
        self.write_endian(it, Endian::Native)
    }
}
impl<const N: usize, W> AsyncWriteExt<N> for W where W: AsyncWrite + Unpin {}

fn assert_future<T, F: Future<Output = T>>(f: F) -> F {
    f
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use crate::{
        io::{ReadExt as _, WriteExt as _},
        tokio::{AsyncReadExt as _, AsyncWriteExt as _},
        Endian,
    };

    use tempfile::NamedTempFile;
    // Performance is _awful_ if we don't use buffered IO
    use tokio::io::{AsyncWriteExt as _, BufReader, BufWriter};
    const LOWER: i64 = -500_000;
    const UPPER: i64 = 500_000;
    const CAPACITY: usize = 1021; // prime

    #[test]
    fn read() {
        for endian in [Endian::Big, Endian::Little] {
            let mut f = NamedTempFile::new().unwrap();
            f.write_endian(1u8, endian).unwrap();
            for it in LOWER..UPPER {
                f.write_endian(it, endian).unwrap()
            }
            f.flush().unwrap();
            let mut f =
                BufReader::with_capacity(CAPACITY, tokio::fs::File::from(f.reopen().unwrap()));
            block_on(async {
                assert_eq!(1u8, f.read_endian(endian).await.unwrap());
                for expected in LOWER..UPPER {
                    let actual = f.read_endian(endian).await.unwrap();
                    assert_eq!(expected, actual)
                }
            });
        }
    }

    #[test]
    fn write() {
        for endian in [Endian::Big, Endian::Little] {
            let mut f = NamedTempFile::new().unwrap();
            block_on(async {
                let mut f =
                    BufWriter::with_capacity(CAPACITY, tokio::fs::File::from(f.reopen().unwrap()));
                f.write_endian(1u8, endian).await.unwrap();
                for i in LOWER..UPPER {
                    f.write_endian(i, endian).await.unwrap();
                }
                f.flush().await.unwrap();
            });
            assert_eq!(1u8, f.read_endian(endian).unwrap());
            for expected in LOWER..UPPER {
                let actual = f.read_endian(endian).unwrap();
                assert_eq!(expected, actual);
            }
        }
    }

    fn block_on<T>(f: impl std::future::Future<Output = T>) -> T {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }
}
