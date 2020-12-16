use std::io;
use std::task::{ Poll, Context };
use std::os::unix::io::AsRawFd;
use tokio::io::unix::AsyncFd;
use crate::fd::{ FdBuilder, AsyncReadyFd };
use crate::TokioBuilder;


impl<T> FdBuilder<T> for TokioBuilder
where
    T: AsRawFd
{
    type Target = AsyncFd<T>;

    #[inline]
    fn build_fd(&self, fd: T) -> io::Result<Self::Target> {
        AsyncFd::new(fd)
    }
}

impl<T> AsyncReadyFd<T> for AsyncFd<T>
where
    T: AsRawFd
{
    fn poll_read_with<F, R>(&self, cx: &mut Context<'_>, f: &mut F) -> Poll<io::Result<R>>
    where
        F: FnMut(&Self) -> io::Result<R>
    {
        match self.poll_read_ready(cx) {
            Poll::Ready(Ok(mut guard)) => guard.with_poll(|| match f(self) {
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => Poll::Pending,
                res => Poll::Ready(res)
            }),
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending
        }
    }

    fn poll_write_with<F, R>(&self, cx: &mut Context<'_>, f: &mut F) -> Poll<io::Result<R>>
    where
        F: FnMut(&Self) -> io::Result<R>
    {
        match self.poll_write_ready(cx) {
            Poll::Ready(Ok(mut guard)) => guard.with_poll(|| match f(self) {
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => Poll::Pending,
                res => Poll::Ready(res)
            }),
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending
        }
    }
}
