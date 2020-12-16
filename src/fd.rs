#[cfg(feature = "tokio-impl")]
pub mod tokio_impl;

#[cfg(feature = "async-io-impl")]
pub mod async_io_impl;

use std::io;
use std::pin::Pin;
use std::future::Future;
use std::marker::PhantomData;
use std::os::unix::io::AsRawFd;
use std::task::{ Poll, Context };
use pin_project_lite::pin_project;


pub trait FdBuilder<T>
where
    T: AsRawFd
{
    type Target: AsyncReadyFd<T>;

    fn build_fd(&self, fd: T) -> io::Result<Self::Target>;
}

pub trait AsyncReadyFd<T>
where
    Self: Sized,
    T: AsRawFd
{
    fn poll_read_with<F, R>(&self, cx: &mut Context<'_>, f: &mut F) -> Poll<io::Result<R>>
    where
        F: FnMut(&Self) -> io::Result<R>;

    fn poll_write_with<F, R>(&self, cx: &mut Context<'_>, f: &mut F) -> Poll<io::Result<R>>
    where
        F: FnMut(&Self) -> io::Result<R>;

    #[inline]
    fn read_with<F, R>(&mut self, f: F) -> ReadWithFuture<'_, T, Self, F, R>
    where
        F: FnMut(&Self) -> io::Result<R>
    {
        ReadWithFuture {
            fd: self,
            func: f,
            _phantom: PhantomData
        }
    }

    #[inline]
    fn write_with<F, R>(&mut self, f: F) -> WriteWithFuture<'_, T, Self, F, R>
    where
        F: FnMut(&Self) -> io::Result<R>
    {
        WriteWithFuture {
            fd: self,
            func: f,
            _phantom: PhantomData
        }
    }
}

pin_project!{
    pub struct ReadWithFuture<'a, T, A, F, R> {
        fd: &'a A,
        func: F,
        _phantom: PhantomData<(T, R)>
    }
}

impl<T, A, F, R> Future for ReadWithFuture<'_, T, A, F, R>
where
    T: AsRawFd,
    A: AsyncReadyFd<T>,
    F: FnMut(&A) -> io::Result<R>
{
    type Output = io::Result<R>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        this.fd.poll_read_with(cx, &mut this.func)
    }
}


pin_project!{
    pub struct WriteWithFuture<'a, T, A, F, R> {
        fd: &'a A,
        func: F,
        _phantom: PhantomData<(T, R)>
    }
}

impl<T, A, F, R> Future for WriteWithFuture<'_, T, A, F, R>
where
    T: AsRawFd,
    A: AsyncReadyFd<T>,
    F: FnMut(&A) -> io::Result<R>
{
    type Output = io::Result<R>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        this.fd.poll_write_with(cx, &mut this.func)
    }
}
