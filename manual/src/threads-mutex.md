# The Mutex Penalty

We still have a few minor potential bottlenecks:

* We know roughly how many results we can expect, so we can give our vectors the appopriate capacity at startup.
* We are using an arc/mutex on the results, requiring a lock on each access to store results.

We can actually go without either!

```rust
use std::time::Instant;
use rand::seq::SliceRandom;

// DELETE THIS:
//const NUM_THREADS: usize = 10;
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
    let num_cpus = num_cpus::get();
    println!("Using {num_cpus} threads.");
    let mut candidates: Vec<usize> = (0..MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation
    let mut primes: Vec<usize> = Vec::with_capacity(10_000);
    candidates.shuffle(&mut rand::thread_rng());

    std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(num_cpus);

        let chunks = candidates.chunks(candidates.len() / num_cpus);
        for chunk in chunks {
            let handle = scope.spawn(move || {
                    chunk
                        .iter()
                        .filter(|n| is_prime(**n))
                        .map(|n| *n)
                        .collect()
            });
            handles.push(handle);
        }

        for handle in handles {
            let local_result: Vec<usize> = handle.join().unwrap();
            primes.extend(local_result);
        }
    });
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", primes.len());
    //println!("{:?}", *primes);
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
```

This yields a small improvement:

```
Using 20 threads.
Found 9592 primes
Calculated in 0.0606 seconds
```

Test | Threads | Time (seconds)
-|-|-
Single Thread | 1 | 0.5633
Simple Chunked | 10 | 0.1180
Use All CPUs | 20 | 0.0861
Shuffled | 20 | 0.0649
No Mutex | 20 | 0.0606
