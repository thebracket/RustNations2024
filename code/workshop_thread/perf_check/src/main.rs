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
        let chunks = candidates.chunks(candidates.len() / num_cpus);
        // We'll use enumerate() to get the index of each chunk
        for (id, chunk) in chunks.enumerate() {
            // And check the chunk sizes
            println!("Thread #{id} is using chunk size: {}", chunk.len());
            println!("Thread #{id} starts at: {}", chunk[0]);
            let my_primes = primes.clone();
            scope.spawn(move || {
                // And time each chunk
                let chunk_start = Instant::now();

                let local_results: Vec<usize> =
                    chunk.iter().filter(|n| is_prime(**n)).map(|n| *n).collect();
                let mut lock = my_primes.lock().unwrap();
                lock.extend(local_results);

                // Print the time for each chunk
                let chunk_elapsed = chunk_start.elapsed();
                println!("Thread #{id} took {:.4} seconds", chunk_elapsed.as_secs_f32());
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
