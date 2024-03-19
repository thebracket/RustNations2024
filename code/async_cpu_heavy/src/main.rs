use tokio::task::JoinSet;

async fn is_prime(n: usize) -> (usize, bool) {
    if n <= 1 {
        (n, false)
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return (n, false);
            }
            if div % 1000 == 0 {
                tokio::task::yield_now().await;
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