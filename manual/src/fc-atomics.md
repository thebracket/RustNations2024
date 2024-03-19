# Atomic Safety

You can try over and over to make the data race work in Rust without using `unsafe`. If you succeed, please let the core team know!

In our example code, the safe thing to do is to use an `Atomic`. Atomics are great---CPUs provide various intrinsics that ensure that an integer operation appears to take only one step, and the value remains consistent between CPU cores. Atomics are also *really fast*.

Here's some code that uses atomic operations:

```rust
use std::{thread, sync::atomic::{AtomicUsize, Ordering}};

const LOOP_COUNTER: usize = 10000;
const N_THREADS: usize = 100;

/// Safety: Hold my beer
fn main() {
    let counter = AtomicUsize::new(0);

    thread::scope(|scope| {
        for _ in 0.. N_THREADS {
            scope.spawn(|| {
                for _ in 0.. LOOP_COUNTER {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });

    println!("{}", counter.load(Ordering::Relaxed));
}
```

It gives `1000000` every single time. Houston, we managed to land anyway!

The Rust syntax for atomics is a little verbose. Rust makes you consider the order of operations. `Ordering::Relaxed` is the weakest guaranty. There's no guaranty at all that the additions will happen in the order in which they were submitted, only that they will occur. If you are doing something that requires strict ordering (such as building a spinlock), `Acquire` and `Release` are a little slower---but offer some safety.

> Check out Mara Bos's excellent `Rust Atomics and Locks` for some really great examples of how this gives you the building blocks for many other concurrent designs.

## Credit Where its Due

C++ wins on the easy atomics front, by the way.

Changing two lines:
* Adding `#include <atomic>`
* Changing `counter` to an `std::atomic_int`

Is all that's required to get the correct answer every time with C++, too.

Go is pretty good, too. You can:

* Import `sync` and `sync/atomic`
* Declare counter as `var counter atomic.Uint64`.
* Increment with `counter.Add(1)`

If either language would tell you about the problem, you could avoid it!

## Superpowers!

So Rust has demonstrated superpowers here that tie into fearless concurrency:

* Data races --- a problem that Uber discovered they had over 1,110 times! --- take an awful lot of work to create.
* Using concurrency primitives in Rust is pretty straightforward
* You have a seatbelt. You can hack away at your concurrent problem and Rust will catch you when you make a mistake.

