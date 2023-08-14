use core::task;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::Stream;
use pin_project::pin_project;

use crate::LendingStream;

/// The iterator returned from `AsyncIterator::lend`.
#[derive(Debug)]
#[pin_project]
pub struct Lend<I: Stream>(#[pin] I);

impl<I: Stream> Lend<I> {
    pub(crate) fn new(i: I) -> Self {
        Self(i)
    }
}

impl<I: Stream + Unpin> LendingStream for Lend<I> {
    type Item<'a> = (&'a I, I::Item)
    where
        Self: 'a;

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item<'_>>> {
        let item = task::ready!(Pin::new(&mut self.0).poll_next(cx));
        Poll::Ready(item.map(move |item| (&self.0, item)))
    }
}
