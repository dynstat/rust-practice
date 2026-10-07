//! Lesson 4: `Pin` keeps a self-referential future at a stable address.
//!
//! ```text
//! cargo run --bin async_l4_pin
//! ```
//!
//! `Countdown` is `Unpin`, so `Pin::new` is safe. An `async` block is
//! `!Unpin`. Uncomment the marked block below and the compiler rejects
//! `Pin::new`. `Box::pin` is the fix: moving the `Box` moves a pointer.
//!
//! Notes: `Notes/ASYNC_RUST_PART_I.md`

use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

fn main() {
    let mut countdown = Countdown { left: 1 };
    let _pinned_countdown = Pin::new(&mut countdown);
    println!("Pin::new accepted Countdown (it is Unpin)");

    // Uncomment to read the compiler error. `async` blocks are `!Unpin`.
    //
    // let mut fut = async { 1 };
    // let _pinned = Pin::new(&mut fut);

    let pinned = Box::pin(async { 1 });
    let message = block_on(pinned);
    println!("Box::pin async block -> {message}");

    let stacked = pin!(async { 2 });
    let message = block_on(stacked);
    println!("pin! async block -> {message}");

    block_on(hold());
}

async fn hold() {
    let name = String::from("vivek");
    let borrowed = &name;
    Countdown { left: 1 }.await;
    println!("borrow across await: {borrowed}");
}

struct Countdown {
    left: u32,
}

impl Future for Countdown {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.left == 0 {
            Poll::Ready(())
        } else {
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
