use std::{thread, sync::atomic::{AtomicUsize, Ordering}};

const LOOP_COUNTER: usize = 10000;
const N_THREADS: usize = 100;

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
