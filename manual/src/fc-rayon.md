# Making it look easy with Rayon

Just in case you thought that all this thread code was messy, you should consider looking at the `rayon` crate. Rayon builds a thread pool, and gives you some very handy shortcuts for using threads without having to worry so much about the internals:

```rust,edition2021
use rayon::prelude::*;

fn main() {
    let numbers = (0 .. 10_000).collect::<Vec<u64>>();
    let total: u64 = numbers
        .par_iter()
        .sum();
    println!("{total}");
}
```

Rayon includes parallel sort, parallel iteration, parallel chunking, and its own task system. It's really powerful. It's not suitable for *every* problem, but when you want to blast through something like summing a vector---it's there, it's easy to use and it's fast!