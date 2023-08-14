//! A lending version of Stream
//!
//! # Examples
//!
//! ```
//! // tbi
//! ```

// #![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub)]

mod lend;
mod lend_mut;
mod next;

use std::task::{Context, Poll};

pub use lend::Lend;
pub use lend_mut::LendMut;
pub use next::Next;

use futures_core::Stream;

/// The lending-stream prelude
pub mod prelude {
    pub use super::LendingStream;
    pub use super::StreamExt as _;
}

/// An extension for the `Stream` trait
pub trait StreamExt: Stream {
    /// Creates a stream which yields a reference to `self` as well as
    /// the next value.
    fn lend(self) -> Lend<Self>
    where
        Self: Sized + Unpin,
    {
        Lend::new(self)
    }
    /// Creates a stream which yields a mutable reference to `self` as well
    /// as the next value.
    fn lend_mut(self) -> LendMut<Self>
    where
        Self: Sized + Unpin,
    {
        LendMut::new(self)
    }
}

impl<S: Stream> StreamExt for S {}

/// An interface for dealing with iterators which borrow from `Self`

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait LendingStream {
    /// The type of the elements being iterated over.
    type Item<'a>
    where
        Self: 'a;

    /// Attempt to pull out the next value of this stream, registering
    /// the current task for wakeup if the value is not yet available, and
    /// returning None if the async iterator is exhausted.
    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item<'_>>>;

    /// Returns the bounds on the remaining length of the Stream.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Retrieves the next item in the stream.
    ///
    /// Returns [`None`] when iteration is finished. Stream implementations may choose to or not to
    /// resume iteration after that.
    fn next(&mut self) -> Next<'_, Self>
    where
        Self: Unpin,
    {
        Next::new(self)
    }
}
