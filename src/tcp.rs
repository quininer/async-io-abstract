#[cfg(feature = "tokio-impl")]
pub mod tokio_impl;

#[cfg(feature = "async-io-impl")]
pub mod async_io_impl;

use std::io;
use std::pin::Pin;
use std::future::Future;
use std::task::{ Poll, Context };
use std::net::{ TcpStream, TcpListener };

#[cfg(feature = "tokio-api")]
use tokio::io::{ AsyncRead as TokioAsyncRead, AsyncWrite as TokioAsyncWrite };

#[cfg(feature = "futures-io-api")]
use futures_io::{ AsyncRead as FuturesAsyncRead, AsyncWrite as FuturesAsyncWrite };


pub trait TcpBuilder {
    #[cfg(all(feature = "tokio-api", not(feature = "futures-io-api")))]
    type Stream: TokioAsyncRead + TokioAsyncWrite;

    #[cfg(all(feature = "futures-io-api", not(feature = "tokio-api")))]
    type Stream: FuturesAsyncRead + FuturesAsyncWrite;

    #[cfg(all(feature = "tokio-api", feature = "futures-io-api"))]
    type Stream: TokioAsyncRead + TokioAsyncWrite + FuturesAsyncRead + FuturesAsyncWrite;

    #[cfg(all(not(feature = "tokio-api"), not(feature = "futures-io-api")))]
    type Stream;

    type Listener: TcpAccepter<Stream = Self::Stream>;

    fn build_stream(&self, stream: TcpStream) -> io::Result<Self::Stream>;
    fn build_listener(&self, listener: TcpListener) -> io::Result<Self::Listener>;
}

pub trait TcpAccepter
where
    Self: Sized
{
    type Stream;

    fn poll_accept(&self, cx: &mut Context<'_>) -> Poll<io::Result<Self::Stream>>;

    #[inline]
    fn accept(&self) -> AcceptFuture<'_, Self> {
        AcceptFuture {
            listener: self
        }
    }
}

pub struct AcceptFuture<'a, T> {
    listener: &'a T
}

impl<T> Future for AcceptFuture<'_, T>
where
    T: TcpAccepter
{
    type Output = io::Result<T::Stream>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.listener.poll_accept(cx)
    }
}
