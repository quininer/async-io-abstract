use std::io;
use std::net::{ TcpStream, TcpListener };
use std::task::{ Poll, Context };
use async_io::Async;
use crate::tcp::{ TcpBuilder, TcpAccepter };
use crate::async_io_util::poll_read_with;
use crate::AsyncIoBuilder;


pub struct TcpStream2(pub Async<TcpStream>);

pub struct TcpListener2(pub Async<TcpListener>);

impl TcpBuilder for AsyncIoBuilder {
    type Stream = TcpStream2;
    type Listener = TcpListener2;

    fn build_stream(&self, stream: TcpStream) -> io::Result<Self::Stream> {
        Async::new(stream).map(TcpStream2)
    }

    fn build_listener(&self, listener: TcpListener) -> io::Result<Self::Listener> {
        Async::new(listener).map(TcpListener2)
    }
}

impl TcpAccepter for TcpListener2 {
    type Stream = TcpStream2;

    fn poll_accept(&self, cx: &mut Context<'_>) -> Poll<io::Result<Self::Stream>> {
        match poll_read_with(&self.0, cx, |io| io.get_ref().accept()) {
            Poll::Ready(Ok((stream, _))) => Poll::Ready(Async::new(stream).map(TcpStream2)),
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending
        }
    }
}

#[cfg(feature = "tokio-api")]
mod tokio_api {
    use super::*;
    use std::pin::Pin;
    use std::io::IoSlice;
    use tokio::io::{ AsyncRead, AsyncWrite, ReadBuf };
    use futures_io::{ AsyncRead as _, AsyncWrite as _ };

    impl AsyncRead for TcpStream2 {
        fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>)
            -> Poll<io::Result<()>>
        {
            match Pin::new(&mut self.0).poll_read(cx, buf.initialize_unfilled()) {
                Poll::Ready(Ok(n)) => {
                    buf.advance(n);
                    Poll::Ready(Ok(()))
                },
                Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                Poll::Pending => Poll::Pending
            }
        }
    }

    impl AsyncWrite for TcpStream2 {
        #[inline]
        fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8])
            -> Poll<io::Result<usize>>
        {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        #[inline]
        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<io::Result<()>>
        {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        #[inline]
        fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<io::Result<()>>
        {
            Pin::new(&mut self.0).poll_close(cx)
        }

        #[inline]
        fn poll_write_vectored(mut self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>])
            -> Poll<io::Result<usize>>
        {
            Pin::new(&mut self.0).poll_write_vectored(cx, bufs)
        }
    }
}

#[cfg(feature = "futures-io-api")]
mod futures_io_api {
    use super::*;
    use std::pin::Pin;
    use std::io::IoSlice;
    use futures_io::{ AsyncRead, AsyncWrite };

    impl AsyncRead for TcpStream2 {
        #[inline]
        fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8])
            -> Poll<io::Result<usize>>
        {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for TcpStream2 {
        #[inline]
        fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8])
            -> Poll<io::Result<usize>>
        {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        #[inline]
        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<io::Result<()>>
        {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        #[inline]
        fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<io::Result<()>>
        {
            Pin::new(&mut self.0).poll_close(cx)
        }

        #[inline]
        fn poll_write_vectored(mut self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>])
            -> Poll<io::Result<usize>>
        {
            Pin::new(&mut self.0).poll_write_vectored(cx, bufs)
        }
    }
}
