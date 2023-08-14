use std::task::Poll;

use futures_core::Stream;
use futures_lite::prelude::*;
use lending_stream::prelude::*;

#[pin_project::pin_project]
struct Hello(String);

impl Stream for Hello {
    type Item = String;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        Poll::Ready(Some(this.0.clone()))
    }
}

fn main() {
    futures_lite::future::block_on(async {
        let mut hello = Hello(String::from("hi hi hi"));
        let _value: String = hello.next().await.unwrap();
        let (_x, _y): (&mut Hello, String) = hello.lend_mut().next().await.unwrap();
    })
}
