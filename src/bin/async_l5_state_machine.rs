//! Lesson 5: `async fn` compiles to an enum with one state per `.await`.
//!
//! `FetchBoth` is that enum, written by hand. `sized_futures` prints why a
//! large value kept alive across `.await` inflates the future.
//!
//! ```text
//! cargo run --bin async_l5_state_machine
//! ```
//!
//! Notes: `Notes/ASYNC_RUST_PART_I.md`

use std::future::Future;
use std::mem::{replace, size_of_val};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

fn main() {
    sized_futures();
    let body = block_on(FetchBoth::Start);
    println!("fetch_both -> {body}");
}

/// Same shape as:
///
/// ```ignore
/// async fn fetch_both() -> String {
///     let page1 = http_get("/a").await;
///     let page2 = http_get("/b").await;
///     format!("{page1}{page2}")
/// }
/// ```
enum FetchBoth {
    Start,
    WaitingFirst { fut1: HttpGet },
    WaitingSecond { page1: String, fut2: HttpGet },
    Done,
}

impl Future for FetchBoth {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        // `FetchBoth` is `Unpin`: its fields do not borrow each other.
        let this = self.get_mut();
        loop {
            match replace(this, FetchBoth::Done) {
                FetchBoth::Start => {
                    println!("state Start -> WaitingFirst");
                    *this = FetchBoth::WaitingFirst {
                        fut1: HttpGet::new("/a"),
                    };
                }
                FetchBoth::WaitingFirst { mut fut1 } => match Pin::new(&mut fut1).poll(cx) {
                    Poll::Pending => {
                        *this = FetchBoth::WaitingFirst { fut1 };
                        return Poll::Pending;
                    }
                    Poll::Ready(page1) => {
                        println!("state WaitingFirst -> WaitingSecond");
                        *this = FetchBoth::WaitingSecond {
                            page1,
                            fut2: HttpGet::new("/b"),
                        };
                    }
                },
                FetchBoth::WaitingSecond { page1, mut fut2 } => {
                    match Pin::new(&mut fut2).poll(cx) {
                        Poll::Pending => {
                            *this = FetchBoth::WaitingSecond { page1, fut2 };
                            return Poll::Pending;
                        }
                        Poll::Ready(page2) => {
                            println!("state WaitingSecond -> Done");
                            let body = format!("{page1}{page2}");
                            *this = FetchBoth::Done;
                            return Poll::Ready(body);
                        }
                    }
                }
                FetchBoth::Done => {
                    // Polling after `Ready` is outside the `Future` contract.
                    panic!("polled FetchBoth after it completed");
                }
            }
        }
    }
}

struct HttpGet {
    path: &'static str,
    polls_left: u8,
}

impl HttpGet {
    fn new(path: &'static str) -> Self {
        Self {
            path,
            polls_left: 1,
        }
    }
}

impl Future for HttpGet {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        if self.polls_left == 0 {
            println!("http_get {} ready", self.path);
            Poll::Ready(format!("body{}", self.path))
        } else {
            println!("http_get {} pending", self.path);
            self.polls_left -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn sized_futures() {
    let small = async { 1u8 };
    let big = holds_page();
    println!("async {{ 1 }} is {} bytes", size_of_val(&small));
    println!(
        "future that holds a 4 KiB array across .await is {} bytes",
        size_of_val(&big)
    );
    // Drop both without polling. Their bodies do not run.
}

async fn holds_page() -> u8 {
    let buf = [0u8; 4096];
    ready_byte().await;
    buf[0]
}

async fn ready_byte() -> u8 {
    0
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
