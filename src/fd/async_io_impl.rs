use std::io;
use std::task::{ Poll, Context };
use std::os::unix::io::AsRawFd;
use async_io::Async;
use crate::fd::{ FdBuilder, AsyncReadyFd };
use crate::async_io_util::{ poll_read_with, poll_write_with };
use crate::AsyncIoBuilder;


impl<T> FdBuilder<T> for AsyncIoBuilder
where
    T: AsRawFd
{
    type Target = Async<T>;

    #[inline]
    fn build_fd(&self, fd: T) -> io::Result<Self::Target> {
        Async::new(fd)
    }
}

impl<T> AsyncReadyFd<T> for Async<T>
where
    T: AsRawFd
{
    #[inline]
    fn poll_read_with<F, R>(&self, cx: &mut Context<'_>, f: &mut F) -> Poll<io::Result<R>>
    where
        F: FnMut(&Self) -> io::Result<R>
    {
        poll_read_with(&self, cx, |io| f(io))
    }

    #[inline]
    fn poll_write_with<F, R>(&self, cx: &mut Context<'_>, f: &mut F) -> Poll<io::Result<R>>
    where
        F: FnMut(&Self) -> io::Result<R>
    {
        poll_write_with(&self, cx, |io| f(io))
    }
}
