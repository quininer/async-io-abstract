#[cfg(unix)]
pub mod fd;
pub mod tcp;


#[cfg(feature = "tokio-impl")]
pub struct TokioBuilder;

#[cfg(feature = "async-io-impl")]
pub struct AsyncIoBuilder;


#[cfg(feature = "async-io-impl")]
pub mod async_io_util {
    use std::io;
    use std::task::{ Poll, Context };
    use async_io::Async;


    pub fn poll_read_with<T, F, R>(a: &Async<T>, cx: &mut Context<'_>, f: F)
        -> Poll<io::Result<R>>
    where
        F: FnMut(&Async<T>) -> io::Result<R>
    {
        let mut op = f;
        loop {
            match op(a) {
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => (),
                res => return Poll::Ready(res),
            }

            match a.poll_readable(cx) {
                Poll::Ready(Ok(())) => (),
                Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
                Poll::Pending => return Poll::Pending
            }
        }
    }

    pub fn poll_write_with<T, F, R>(a: &Async<T>, cx: &mut Context<'_>, f: F)
        -> Poll<io::Result<R>>
    where
        F: FnMut(&Async<T>) -> io::Result<R>
    {
        let mut op = f;
        loop {
            match op(a) {
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => (),
                res => return Poll::Ready(res),
            }

            match a.poll_writable(cx) {
                Poll::Ready(Ok(())) => (),
                Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
                Poll::Pending => return Poll::Pending
            }
        }
    }
}
