# Use All Your CPUs

You may not have exactly 10 CPUs. So let's change things up to use as many CPUs as you have. I'm on a tiny MacBook Air M1, so this may not help me. If you happen to have an Epyc lying around, this would make a huge difference!

Let's add a dependency to a crate called `num_cpus` (if the Internet isn't working, as sometimes happens at conventions - change the constant!)

```bash
cargo add num_cpus
```

Now we can change our code to use all of our CPUs and report how many it is using:

```rust,,edition2021
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

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
    let candidates: Vec<usize> = (0..MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation
    let primes: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    std::thread::scope(|scope| {
        let chunks = candidates.chunks(num_cpus);
        for chunk in chunks {
            let my_primes = primes.clone();
            scope.spawn(move || {
                let local_results: Vec<usize> =
                    chunk.iter().filter(|n| is_prime(**n)).map(|n| *n).collect();
                let mut lock = my_primes.lock().unwrap();
                lock.extend(local_results);
            });
        }
    });
    let elapsed = start.elapsed();

    // Results
    let lock = primes.lock().unwrap();
    println!("Found {} primes", lock.len());
    //println!("{:?}", *lock);
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
```

So on the computer I used to write this, I now get:

```
Using 20 threads.
Found 9592 primes
Calculated in 0.0867 seconds
```

Test | Threads | Time (seconds)
-|-|-
Single Thread | 1 | 0.5633
Simple Chunked | 10 | 0.1180
Use All CPUs | 20 | 0.0861

It's not a linear speed-up, but it's even faster.