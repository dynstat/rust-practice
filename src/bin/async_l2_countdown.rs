//! Lesson 2: `Future::poll` returns `Ready` or `Pending`.
//!
//! Each `Pending` calls `wake_by_ref` so the executor polls again.
//! That wake is immediate, so this is a busy countdown, not a timer.
//!
//! ```text
//! cargo run --bin async_l2_countdown
//! ```
//!
//! The executor loop is explained in `async_l3_executor`.
//! Notes: `Notes/ASYNC_RUST_PART_I.md`

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

fn main() {
    let message = block_on(Countdown::new(3));
    println!("done: {message}");
}

struct Countdown {
    left: u32,
}

impl Countdown {
    fn new(from: u32) -> Self {
        Self { left: from }
    }
}

impl Future for Countdown {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.left == 0 {
            println!("liftoff");
            Poll::Ready("liftoff")
        } else {
            println!("{}...", self.left);
            self.left -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
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
