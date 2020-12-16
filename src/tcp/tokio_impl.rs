use std::io;
use std::net::{ TcpStream as StdTcpStream, TcpListener as StdTcpListener };
use std::task::{ Poll, Context };
use tokio::net::{ TcpStream, TcpListener };
use crate::tcp::{ TcpBuilder, TcpAccepter };
use crate::TokioBuilder;


pub struct TcpStream2(pub TcpStream);

pub struct TcpListener2(pub TcpListener);

impl TcpBuilder for TokioBuilder {
    type Stream = TcpStream2;
    type Listener = TcpListener2;

    fn build_stream(&self, stream: StdTcpStream) -> io::Result<Self::Stream> {
        TcpStream::from_std(stream).map(TcpStream2)
    }

    fn build_listener(&self, listener: StdTcpListener) -> io::Result<Self::Listener> {
        TcpListener::from_std(listener).map(TcpListener2)
    }
}

impl TcpAccepter for TcpListener2 {
    type Stream = TcpStream2;

    fn poll_accept(&self, cx: &mut Context<'_>) -> Poll<io::Result<Self::Stream>> {
        match self.0.poll_accept(cx) {
            Poll::Ready(Ok((stream, _))) => Poll::Ready(Ok(TcpStream2(stream))),
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

    impl AsyncRead for TcpStream2 {
        #[inline]
        fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>)
            -> Poll<io::Result<()>>
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
        fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<io::Result<()>>
        {
            Pin::new(&mut self.0).poll_shutdown(cx)
        }

        #[inline]
        fn poll_write_vectored(mut self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>])
            -> Poll<io::Result<usize>>
        {
            Pin::new(&mut self.0).poll_write_vectored(cx, bufs)
        }

        #[inline]
        fn is_write_vectored(&self) -> bool {
            self.0.is_write_vectored()
        }
    }
}

#[cfg(feature = "futures-io-api")]
mod futures_io_api {
    use super::*;
    use std::pin::Pin;
    use std::io::IoSlice;
    use futures_io::{ AsyncRead, AsyncWrite };
    use tokio::io::{ AsyncRead as _, AsyncWrite as _, ReadBuf };

    impl AsyncRead for TcpStream2 {
        #[inline]
        fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8])
            -> Poll<io::Result<usize>>
        {
            let mut buf = ReadBuf::new(buf);

            match Pin::new(&mut self.0).poll_read(cx, &mut buf) {
                Poll::Ready(Ok(())) => Poll::Ready(Ok(buf.filled().len())),
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
        fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<io::Result<()>>
        {
            Pin::new(&mut self.0).poll_shutdown(cx)
        }

        #[inline]
        fn poll_write_vectored(mut self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>])
            -> Poll<io::Result<usize>>
        {
            Pin::new(&mut self.0).poll_write_vectored(cx, bufs)
        }
    }
}
