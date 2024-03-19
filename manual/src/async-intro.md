# Asynchronous Programming

We just saw how Rust makes it easy to unleash your CPU power, either directly with threads or through a wrapper such as Rayon. Rust is *great* for powering through CPU-bound problems, and Rust's synchronous syntax and hand-holding makes for a pretty happy Rustacean experience.

So what about async? Async sometimes gets a bit of a bad name. There's a few reasons for this:

* Async was added to the Rust language later.
* Async isn't really "finished"---you can work wonders with it already, but there's rough edges that can be very frustrating.
    * Things like the recently-stabilied "async traits" are gradually making this easer.
    * Some changes will require an edition change and some syntax-breaking changes.
* Async was designed to be agnostic. It doesn't make in a runtime.
    * Go, Erlang, C#, Java, etc. are all *opinionated*. The runtime builds an environment, and you better like it!
        * This allows for amazing performance for the language's task of choice.
        * Go's "green threads" for example are async on steroids, dividing workload between CPU threads and automatically yielding when a task is taking too long. It also imposes garbage collection overhead making it a tough choice for latency-sensitive CPU-bound tasks.
    * Rust and C++ are agnostic. The language provides all the constructs needed to make async/coroutines work, but it's up to the user to pick a runtime/executor.
        * This allows you to choose Tokio for a nearly Go-like thread-per core, work-stealing environment (without split stacks or automatic preemption).
        * Or you can choose "smol" and scale all the way down to a tiny environment.
        * Or you can choose `glomio` and go Linux-only and make use of `io_uring` (Tokio is adding that option, too) for OS-assisted async file operations.
