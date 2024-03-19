# Async Execution Model

Who here is old enough to remember the Archimedes, Amiga, Atari ST, Mac OS prior to 10, or Windows prior to '95? All of these used *cooperative multitasking*. A program responded to an event, ran for a bit, and when the event is completed---or the task called its equivalent of "yield"---the next program got its timeslice.

You may remember being frustrated that a program would take too long, and slow everything else down.

You may also remember how amazing it was that you could run some really heavyweight programs and have a responsive system. Lightwave on the Amiga was amazing!

Async can run on a single thread, and still handle thousand of customers. `NodeJS` uses this model!

## The Basics

Take the following async program:

```rust,edition2021
async fn say_hello(n: i32) {
    println!("Hello {n}");
}

async fn tasks() {
    for i in 0..10 {
        tokio::spawn(say_hello(i));
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tasks().await;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
```

Let's take this apart:

* `tokio::main` is a macro that writes a `fn main` for you. The function calls `block_on` in Tokio, to start the async environment with your `async fn main` function.
* We've specified single-threaded execution. You can remove that to have 1 thread per core.
* `tasks()` actually returns a `Future`.
    * The `async` keyword actually modifies your function to return `Future<T>` - where `T` is what your function returns.
    * A `Future` represents a unit of work to perform at some point.
    * You can cancel futures by dropping them - they won't execute.
    * Futures don't actually do anything until you `await` them.
* When you `await` the tasks future:
    * Your function yields control.
    * Tokio marks your function as "waiting", and moves it to the back of the thread's task list.
    * Tokio adds the future to the thread's "work list" - tasks waiting to do something.
    * Tokio runs the next available task in the thread's task list - which will be `tasks` in this case.
* The `tasks` function is calling `tokio::spawn`, which immediately launches futures. So it adds 10 "say hello" to the task list.
* The main thread tells Tokio that it wants to wait for a second, and yields control with `await`.
* This gives the tasks time to run.

So that's nothing you couldn't do with threads (and async is pretty pointless in this case) - but we're doing it all on one thread.

Where this would shine is a complex web service:

* Client sends a request, say "give me all the users".
* The web server receives the TCP connection, and spawns an async task to handle it.
* The async task requests the TCP connection give it the request header. This `awaits`.
* Once the request header is sent, the TCP connection wakes up and reads it. This is sent to the appropriate handler (many systems `await` here, too).
* The handler sends out a database request and `await`s the response.
* The database request arrives, so the handler wakes up, does something with the data and returns it for the server to respond.
* The server formats it into the appropriate web Response format.
* The TCP connection handler wakes up, sends the response, and `awaits` the result.
* Once the TCP connection has sent the data, the TCP connection task wakes up again - and possibly terminates.

So a simple "GET /users" can generate 5 "await" points in which the server is *waiting for something else* - input/output. If you generated a thread per connection, the OS scheduler would be juggling a lot of threads - and context switches (Linux is pretty smart about not scheduling threads that are waiting for IO, but they still consume resources).

If you had 100 connections going at once:

* With a threaded model, you'd have 101 threads (1 for receiving the connection).
* With an async model, you might only have 1 thread.

This works because async tasks only do work in bursts and spend a lot of their time sitting idle waiting for something else - the database, the network, the remote client, etc.
