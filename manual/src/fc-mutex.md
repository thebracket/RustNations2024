# Mutexes

Unfortunately, not everything will fit in an atomic. It would be nice, but atomics are limited to primitives.

Mutexes are *much* slower than atomics, so if you can get by with an atomic you should.

Here's a Rust program that uses a `Mutex`` to do the same thing:

```rust
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
```

> (Show of hands - who is familiar with a Mutex). If not many, then explain that a Mutex is a structure that protects data. Only one consumer can hold the `lock` to a Mutex at a time. If you try to lock a Mutex while it is locked, your thread waits until the lock is relinquished.

So this is a win for Rust: `Mutex` is really easy to use. If you use the verison from `parking_lot`, you can even skip the `unwrap`!

There's a *second* and *third* win hidden in here, too! Take a look at this C++:

```cpp
#include <iostream>
#include <thread>
#include <mutex>

int main() {
    std::mutex mutex;
    int counter = 0;
    std::thread t1([&counter, &mutex]() {
        for (int i = 0; i < 1000000; ++i) {
            std::lock_guard<std::mutex> guard(mutex);
            ++counter;
        }
    });
    std::thread t2([&counter, &mutex]() {
        for (int i = 0; i < 1000000; ++i) {
            std::lock_guard<std::mutex> guard(mutex);
            ++counter;
        }
    });
    t1.join();
    t2.join();

    std::cout << counter << std::endl;

    return 0;
}
```

This is safe code and runs. We've declared a mutex, and lock it every time. If you comment out one of the `std::lock_guard<std::mutex> guard(mutex);` the program stops being safe --- and still compiles without warning. That's the second win. What's the third? It's related---Rust mutexes are attached to the structure they protect. Not only is it impossible to forget to obtain a lock, the syntax is as simple as `.lock()`.

For completeness, Go has mutexes too:

```go
package main

import (
	"fmt"
	"sync"
	"time"
)

// SafeCounter is safe to use concurrently.
type SafeCounter struct {
	mu sync.Mutex
	v  map[string]int
}

// Inc increments the counter for the given key.
func (c *SafeCounter) Inc(key string) {
	c.mu.Lock()
	// Lock so only one goroutine at a time can access the map c.v.
	c.v[key]++
	c.mu.Unlock()
}

// Value returns the current value of the counter for the given key.
func (c *SafeCounter) Value(key string) int {
	c.mu.Lock()
	// Lock so only one goroutine at a time can access the map c.v.
	defer c.mu.Unlock()
	return c.v[key]
}

func main() {
	c := SafeCounter{v: make(map[string]int)}
	for i := 0; i < 1000; i++ {
		go c.Inc("somekey")
	}

	time.Sleep(time.Second)
	fmt.Println(c.Value("somekey"))
}
```

I'll let you make your own comments on the syntax, but notice the potential for shooting yourself in the foot:

* The mutex and what it protects are separate entities. Nothing forces you to remember to use it.
* You have to manually `Unlock` the mutex when you're done with it. We'll be going there in a moment.
