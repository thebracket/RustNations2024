# Let's Do Some Threading!

To ensure you are all comfortable with the concepts here, let's build a simple threaded application. You won't hear that in many other language venues - *simple* and *threaded* don't often live together.

> A programmer had a problem. He thought to himself, "I know, I'll solve it with threads!". Now he has 12 problems.

In this short workshop we will:

1. Define a function that detects if a number is prime. We'll do it quite inefficiently!
2. We'll generate a list of candidate numbers.
3. We'll divide the list into "chunks" of roughly equal size.
4. We'll spawn `n` threads, each of which will tackle a chunk of the data.
5. If a thread determines a number to be prime, it'll be appended to a shared vector.
6. We output execution time and list of primes to `stdout`.

Let's build a single-threaded version and establish our workload.

## Starting Single Threaded

Let's start with a simple single-threaded version. 

```rust
use std::time::Instant;

//const NUM_THREADS: usize = 10;
const MAX_NUMBER: usize = 100_000;

/// Really inefficient prime number calculator
fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation
    let primes: Vec<usize> = candidates
        .iter()
        .filter(|n| is_prime(**n))
        .map(|n| *n)
        .collect();
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", primes.len());
    //println!("{primes:?}");
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
```

Running this locally, I get:

```
Found 9592 primes
Calculated in 0.5633 seconds
```

## Let's try to divide the workload

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
        let chunks = candidates.chunks(NUM_THREADS);

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
Calculated in 0.1406 seconds
```

Comparing those times:

Test | Time (seconds)
-|-
Single Thread | 0.5633
Simple Chunked | 0.1406

So we've achieved a pretty great speed improvement already. You can increase the number of threads you use to match the number of CPU cores you have.