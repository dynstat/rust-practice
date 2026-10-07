//! Lesson 3: an executor is a loop that polls, then parks until `wake`.
//!
//! ```text
//! cargo run --bin async_l3_executor
//! cargo run --bin async_l3_executor -- --hang
//! ```
//!
//! `--hang` returns `Pending` without calling `wake`. `thread::park` never
//! resumes. Press Ctrl+C. A spinning executor (`yield_now` on every
//! `Pending`) would hide this bug, because it polls again anyway.
//!
//! Notes: `Notes/ASYNC_RUST_PART_I.md`

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

fn main() {
    let hang = std::env::args().any(|arg| arg == "--hang");
    if hang {
        println!("parking with no wake; press Ctrl+C");
        block_on(Countdown {
            left: 3,
            signal: false,
        });
        return;
    }

    let message = block_on(Countdown {
        left: 3,
        signal: true,
    });
    println!("done: {message}");
}

struct Countdown {
    left: u32,
    signal: bool,
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
            if self.signal {
                cx.waker().wake_by_ref();
            }
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

/// Poll `future` until it is ready.
///
/// `Wake` unparks this thread. `thread::park` is sticky: an `unpark` that
/// happens during `poll` makes the following `park` return immediately.
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
