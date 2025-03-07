use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread;

// Custom waker struct to hold the thread to unpark
struct MyWaker {
    thread: thread::Thread,
}

// Implement the Wake trait for MyWaker
impl Wake for MyWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }
}

// A simple block_on function to run a future to completion
fn block_on<F: Future>(future: F) -> F::Output {
    let my_waker = Arc::new(MyWaker {
        thread: thread::current(),
    });
    let waker = Waker::from(my_waker);
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => thread::park(),
        }
    }
}

// An example async function
async fn hello() {
    println!("Hello, world!");
}

fn main() {
    block_on(hello());
}
