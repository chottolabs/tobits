### Key Points
- Async in Rust enables non-blocking operations using the `async`/`await` syntax, introduced in Rust 1.39 for easier asynchronous programming.
- It seems likely that to run async code, an executor is needed, typically from external crates like `futures` or `Tokio`, but a minimal example without dependencies is possible with a custom executor.
- The evidence leans toward using established crates for practical applications, but for educational purposes, a simple standard library-based example can demonstrate the concept.

---

### What is Async in Rust?

Async programming in Rust allows your program to handle multiple tasks concurrently without blocking, which is great for operations like waiting for network responses. It uses the `async` keyword to define functions that return a `Future`, a type representing a value that may not be ready yet. The `await` keyword is used within these functions to pause execution until the future is ready, letting other tasks run in the meantime.

### History and Evolution

Before Rust 1.39, async programming relied on libraries like `futures` or `Tokio`, using callbacks or manual future polling. The async/await syntax, stabilized in November 2019, made asynchronous code look more like synchronous code, improving readability and maintainability. This evolution was part of a broader effort to make Rust suitable for high-performance, concurrent systems, with ongoing improvements discussed in community resources like [the Rust blog](https://blog.rust-lang.org/2019/11/07/Async-await-stable.html).

This example defines an async function `hello` and a custom `block_on` function to run it, using only the standard library. However, note that this is more for educational purposes; for real applications, using crates like `futures` is recommended for robustness.

---

### Survey Note: Detailed Exploration of Async in Rust

This section provides a comprehensive analysis of async programming in Rust, including its historical context, technical implementation, and practical considerations, expanding on the direct answer with detailed insights.

#### Background and Historical Context

Asynchronous programming has been a cornerstone of modern systems, especially for I/O-bound tasks like network requests or file operations. In Rust, the journey began with early libraries like `futures` and `Tokio`, which provided mechanisms for handling asynchronous operations through callbacks or manual polling of futures. The stabilization of async/await in Rust 1.39, as detailed in [Async-await on stable Rust](https://blog.rust-lang.org/2019/11/07/Async-await-stable.html), marked a significant milestone. This feature, proposed by Aaron Turon and Alex Crichton in 2016, transformed how developers write concurrent code, making it more readable and maintainable. The async book, available at [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/), offers extensive documentation on this evolution.

#### Technical Implementation

In Rust, async functions are defined with the `async` keyword, which desugars to a state machine implementing the `Future` trait from the standard library's `std::future` module. This trait has a single method, `poll`, which is called by an executor to check if the future is ready. The `await` keyword within async functions pauses execution, yielding control back to the executor if the future is not ready, and resumes when it can make progress.

To run these futures, an executor is essential. The standard library does not provide a built-in executor, leading to reliance on external crates like `futures` for functions like `block_on` or `Tokio` for more complex runtime features. However, for educational purposes, it's possible to implement a simple executor using standard library components, as shown in the direct answer.

#### Minimal Example Without Dependencies: Detailed Breakdown

The minimal example provided avoids external dependencies by implementing a custom executor. Let's break it down:

- **Async Function Definition:** The `hello` function is a simple async function that prints "Hello, world!". It returns a future that, when polled, executes the print statement and completes.

- **Custom Waker Implementation:** The `MyWaker` struct and its associated functions create a waker that can unpark the current thread when the future is ready. This uses `Arc` and `Mutex` for thread-safe sharing, and `Waker::from` to create a waker from a closure that unparks the thread.

- **Block_on Function:** The `block_on` function takes a future, polls it in a loop using a context with the custom waker, and parks the thread when the future is pending. This mimics the behavior of `futures::executor::block_on` but uses only standard library features.

However, this example has limitations. For instance, it doesn't handle non-blocking I/O, as the standard library lacks async I/O primitives without external crates. To demonstrate true async behavior, you'd need futures that can yield control, which typically requires libraries like `Tokio` for async I/O operations.

#### Practical Considerations and Recommendations

While the minimal example is educational, practical async programming in Rust often involves using established crates. For instance, [Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) provides a runtime for async I/O, networking, and scheduling, making it suitable for production systems. Similarly, [async-std](https://async-std.github.io/async-std/) offers a standard library-like API for async operations. These crates handle complexities like thread pools, reactors, and compatibility layers, which are beyond the scope of a minimal standard library implementation.

#### Comparative Analysis: Standard Library vs. External Crates

To illustrate the trade-offs, consider the following table comparing the minimal example with using `futures`:

| Aspect                  | Minimal Example (Standard Library) | Using `futures` Crate         |
|-------------------------|------------------------------------|-------------------------------|
| Dependencies            | None                              | Requires `futures` crate      |
| Executor Implementation | Custom, basic                     | Provided, robust              |
| Async I/O Support       | Limited, blocking operations only | Full, non-blocking I/O        |
| Ease of Use             | Complex, educational              | Simple, production-ready      |
| Performance             | Basic, may block thread           | Optimized, non-blocking       |

This table highlights that while the minimal example is self-contained, it lacks the features and efficiency of established crates, making it less suitable for real-world applications.

#### Unexpected Detail: Educational Value of Minimal Examples

An unexpected benefit of exploring minimal async examples without dependencies is the deep understanding they provide of Rust's async model. By implementing your own executor, you gain insight into how futures are polled, wakers work, and threads are managed, which is not as apparent when using high-level crates. This can be particularly enlightening for developers new to async programming, offering a hands-on approach to learning.

#### Conclusion

In summary, async in Rust is a powerful feature for concurrent programming, with a rich history leading to the current async/await syntax. While a minimal example without dependencies is possible using standard library features, it's more educational than practical, with real-world applications benefiting from crates like `futures` or `Tokio`. The provided example demonstrates the concept, but for production, consider leveraging the ecosystem's established tools.

---

### Key Citations
- [Async-await on stable Rust](https://blog.rust-lang.org/2019/11/07/Async-await-stable.html)
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
- [Tokio Hello World Tutorial](https://tokio.rs/tokio/tutorial/hello-tokio)
- [Understanding async Rust from the bottom up](https://users.rust-lang.org/t/understanding-async-rust-from-the-bottom-up-using-std-only/63872)
- [How to think about async/await in Rust](https://news.ycombinator.com/item?id=36570321)
- [Async in Traits Workarounds](https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html)

#### **How It Works**

1. **`MyWaker` and `Wake` Implementation:**
   - `MyWaker` stores the current thread using `thread::current()`.
   - The `wake` method, required by the `Wake` trait, unparks the thread when the future is ready to proceed.
   - Wrapping `MyWaker` in an `Arc` allows it to be safely shared, and `Waker::from` converts `Arc<MyWaker>` into a `Waker` since `MyWaker` implements `Wake`.

2. **`block_on` Function:**
   - Takes a future `F` by value and pins it using `Box::pin`, resulting in a `Pin<Box<F>>`.
   - Creates a `Waker` from `Arc<MyWaker>` and a `Context` to pass to the future’s `poll` method.
   - Polls the future in a loop:
     - If `Poll::Ready(value)` is returned, it exits with the value.
     - If `Poll::Pending` is returned, it parks the thread, waiting for the waker to unpark it when the future is ready.

3. **Example Usage:**
   - The `hello()` async function simply prints a message.
   - `block_on(hello())` runs it to completion, printing "Hello, world!".
