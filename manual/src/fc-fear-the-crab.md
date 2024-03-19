# Fearful Concurrency with Rust?????

That's probably not the heading you expected. You can write the exact same bug in Rust:

```rust
use std::thread;

const LOOP_COUNTER: usize = 10000;
const N_THREADS: usize = 100;

/// Safety: Hold my beer
fn main() {
    static mut COUNTER: usize = 0;

    thread::scope(|scope| {
        for _ in 0.. N_THREADS {
            scope.spawn(|| {
                for _ in 0.. LOOP_COUNTER {
                    unsafe {
                        COUNTER += 1;
                    }
                }
            });
        }
    });

    unsafe {
        println!("{COUNTER}");
    }
}
```

Running the program gives you the exact same problem:

```bash
cargo run
> 106130
cargo run
> 183828
```

But wait... the only way I could get this bug to compile was to use two dirty tricks:

* `COUNTER` is a `static mut`. Making it static means I don't have to try and borrow it when I access it inside the thread.
* Rust knows all about this trick, and made me add `unsafe` around every access to the `static mut`.

> There *are* legitimate uses for this construct. This isn't one of them. Please don't do this in any production code!

So it would seem we've inadvertently stumbled upon another Rust superpower:

* You can use `unsafe` to emulate the lack of safety in other languages. That can be useful when working directly with hardware.
* Labelling your potentially dangerous code with `unsafe` gives code reviewers a target to home in on. Your code may in fact be perfectly safe and just taking advantage of a performance jump (or relying on externals that can't be safety checked). But its good to label them!
