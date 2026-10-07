//! Lesson 1: an `async fn` builds a `Future` and does not run it.
//!
//! ```text
//! cargo run --bin async_l1_lazy
//! cargo run --bin async_l1_lazy -- --drive
//! ```
//!
//! Notes: `Notes/ASYNC_RUST_PART_I.md`

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

fn main() {
    let drive = std::env::args().any(|arg| arg == "--drive");
    if drive {
        block_on(hello());
        println!("main is done");
        return;
    }

    // `hello()` returns a state machine. Nothing polls it, so `main` drops it.
    let _fut = hello();
    println!("main is done; hello() was never polled");
}

async fn hello() {
    println!("hello from async");
}

struct ThreadWaker {
    thread: Thread,
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWaker {
        thread: thread::current(),
    }));
    let mut cx = Context::from_waker(&waker);
    // SAFETY: `future` stays in this stack slot until `block_on` returns.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => thread::park(),
        }
    }
}
