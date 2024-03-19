use std::{thread, sync::Mutex};

const LOOP_COUNTER: usize = 10000;
const N_THREADS: usize = 100;

struct Counter(usize);

fn main() {
    let counter = Mutex::new(Counter(0));

    thread::scope(|scope| {
        for _ in 0.. N_THREADS {
            scope.spawn(|| {
                for _ in 0.. LOOP_COUNTER {
                    let mut lock = counter.lock().unwrap();
                    lock.0 += 1;
                }
            });
        }
    });


    println!("{}", counter.lock().unwrap().0);
}
