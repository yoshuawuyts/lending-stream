use crate::LendingStream;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Future for the [`StreamExt::next()`] method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
// #[pin_project]
pub struct Next<'a, S: ?Sized + Unpin> {
    stream: &'a mut S,
}

impl<'a, S: ?Sized + Unpin> Next<'a, S> {
    /// Create a new instance of `Next`.
    pub(crate) fn new(stream: &'a mut S) -> Self {
        Self { stream }
    }
}

impl<S: Unpin + ?Sized> Unpin for Next<'_, S> {}

impl<'a, S: LendingStream + Unpin + ?Sized> Future for Next<'a, S> {
    type Output = Option<S::Item<'a>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { stream } = self.get_mut();
        // SAFETY: this seems to be the only way to read from the pointer
        // without getting lifetime errors. We know this should be possible
        // because we have a sugared version of this in
        // `async_iterator::LendingIterator`. And from the documentation of
        // `ptr::read` it doesn't seem like we're violating any invariants, nor are
        // we returning any wrong lifetimes.
        unsafe { std::ptr::read(stream) }.poll_next(cx)
    }
}
