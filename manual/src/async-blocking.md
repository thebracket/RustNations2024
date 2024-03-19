# Blocking

So that's all fine and dandy---you can achieve tremendous IO performance on a single thread. The *downside* is that to benefit from async, you have to play by async's rules.

Let's take some pretty safe looking code:

```rust,edition2021,editable
async fn wait() {
    std::thread::sleep(std::time::Duration::from_secs_f32(0.1))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let now = std::time::Instant::now();

    tokio::join!(
        wait(),
        wait(),
        wait(),
    );

    let elapsed = now.elapsed();
    println!("Time: {:.1}", elapsed.as_secs_f32());
}
```

Running it shows that it took `0.3` seconds. We didn't have any parallelism at all! Well, we're single threaded. So let's edit that (isn't `mdbook` cool?) to remove `(flavor = "current_thread")` and run it again. STILL 0.3 seconds.

That's because calling `thread::sleep` makes the *whole thread* sleep, *including Tokio's task list!*. Oops.

So when you want to sleep in an `async` world, you have to use an async friendly sleep (which yields the task, and tells the executor not to wake it up until its ready):

```rust,edition2021
async fn wait() {
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let now = std::time::Instant::now();

    tokio::join!(
        wait(),
        wait(),
        wait(),
    );

    let elapsed = now.elapsed();
    println!("Time: {:.1}", elapsed.as_secs_f32());
}
```

And we run this---execution time is `0.1`. We've concurrently slept, even though we're on one thread and nothing is running in parallel.

How about CPU bound workloads?

```rust,edition2021,editable
use tokio::task::JoinSet;

async fn is_prime(n: usize) -> (usize, bool) {
    if n <= 1 {
        (n, false)
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return (n, false);
            }
        }
        (n, true)
    }
}

//#[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() {
    let mut candidates: Vec<usize> = (0 .. 500_000).collect();

    let now = std::time::Instant::now();
    let mut tasks = JoinSet::new();
    for i in candidates.drain(..) {
        tasks.spawn(is_prime(i));
    }

    let mut result = Vec::new();
    while let Some(Ok((n, is_prime))) = tasks.join_next().await {
        if is_prime {
            result.push(n);
        }
    }
    let elapsed = now.elapsed();
    println!("Found {} primes in Time: {:.4}", result.len(), elapsed.as_secs_f32());
}
```

Candidates|Flavour|Time (s)
-|-|-
500,000|Current Thread|12.4824
500,000|Default Threading|1.1994
500,000|Rayon from the Workshop|1.1696

So Tokio really doesn't do badly in this case for raw speeds. Raw threads are still a touch faster (and its a fair test, since both Tokio and Rayon get to do setup ahead of the timing).

**BUT** (you knew this was coming)

a) We're not testing the same thing. A threaded test would spawn 500,000 threads and run each one independently. We're just spawning lightweight tasks. You couldn't even start 500,000 threads!
b) If you had *other* tasks running in Tokio at the same time, they might have to wait while the program spins through thousands of iterations.

The latter is the kicker. Async tasks don't yield to other tasks until they either `await` something, or explicitly call `tokio::task::yield_now()`. Since each execution thread (1 per core, or 1 total) is only running a single task at a time, it's quite possible that an I/O event from another task would have to wait behind thousands of other tasks before it gets its turn.

On the other hand, if you add `yield_now` to every iteration of the loop - performance absolutely falls part. Yielding is *much* faster than a thread context switch - but it's not free. So adding the "yield" penalty to every iteration of the loop took the execution (multi threaded) time to 46 seconds on my workstation!

Ouch.

So what can you do?

* You can use Tokio's `spawn_blocking` to spawn a CPU heavy task in a thread. The thread will run as a system thread, and the calling async block will be idle until the thread is finished (you can also detach it if you don't want to wait for the result). That works, but you need to be a bit careful not to swamp all your CPUs.
* You can use some logic and experimentation to find the optimal timeframe to call `yield_now`. Replacing `yield_now` on every iteration with `if div % 1000 == 0 { tokio::task::yield_now().await; }` brought execution time back down to 1.4557 seconds for me. Still slower, but now we're being nice to the underlying async architecture and giving other tasks a chance to run.
* You can stick around for the last part of this workshop, where we'll discuss this problem a bit more!

