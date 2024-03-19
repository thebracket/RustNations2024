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
