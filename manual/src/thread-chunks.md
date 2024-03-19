# Divide the Workload

Now we'll work through dividing the workload up and running each section in its own
scoped thread.

```rust
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

const NUM_THREADS: usize = 10;
const MAX_NUMBER: usize = 100_000;

/// Really inefficient prime number calculator
fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2..n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let candidates: Vec<usize> = (0..MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation

    // Reasoning:
    // * We only want a single list of results, and it must be shared. So `Arc` fits
    //   with what we learned earlier.
    // * We need concurrent access to write the results out at the end, so we
    //   wrap the results list in a Mutex.
    let primes: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    // We're using Scoped threads to make it easy to borrow from the local
    // scope and avoid needing a join block.
    std::thread::scope(|scope| {
        // "Chunks" is a vector function that returns equal-sized chunks
        // referencing vector slices. The last slice will be unequal
        // unless N_THREADS happens to divide cleanly into your max number.
        let chunks = candidates.chunks(candidates.len() / NUM_THREADS);

        // Iterate each chunk
        for chunk in chunks {
            // We make our very own Arc link to the results list, using clone().
            // Clone is cheap for Arc - it just increases the reference counter.
            let my_primes = primes.clone();
            scope.spawn(move || {
                // Perform the same filter/map/collect chain
                // as we did single-threaded
                let local_results: Vec<usize> =chunk
                    .iter()
                    .filter(|n| is_prime(**n))
                    .map(|n| *n)
                    .collect();

                // Lock the shared results list
                let mut lock = my_primes.lock().unwrap();

                // Extend the results with our discovered primes
                lock.extend(local_results);
            });
        }

        // The scope will automatically wait for child threads to finish
        // here.
    });
    // Time how long it took
    let elapsed = start.elapsed();

    // Results
    let lock = primes.lock().unwrap();
    println!("Found {} primes", lock.len());
    //println!("{:?}", *lock);
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
```

Running this locally, I get:

```
Found 9592 primes
Calculated in 0.1180 seconds
```

Comparing those times:

Test | Time (seconds)
-|-
Single Thread | 0.5633
Simple Chunked | 0.1180

So we've achieved a pretty great speed improvement already. We're not breaking any records (ignoring that we're using a terrible prime number detection function - the idea is to heat up the room with your CPUs!)