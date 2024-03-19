# What's going on here?

```cpp
// Initialize a shared counter
int counter = 0;

// Vector (just like Rust's Vec!) to store thread handles
std::vector<std::thread> handles;

// Loop N_THREADS times
for (int i = 0; i < N_THREADS; ++i) {
    // Spawn a thread with `std::thread`
    // Add the join handle to the vector
    handles.push_back(std::thread([&counter]() {
        for (int i = 0; i < N_LOOPS; ++i) {
            // Add 1 to counter.
            ++counter;
        }
    }));
}

// Wait for all threads to finish
for (auto& handle : handles) {
    handle.join();
}

// Print the result
std::cout << counter << std::endl;
```

The problem is that C++ happily lets us add 1 to the shared `counter` variable inside each thread, with no checks that doing so is safe.

Incrementing an integer *is not a safe operation* on shared data. It's actually a multi-stage task:

* Load `counter` into a register.
* Increment the register.
* Store the register back into the bit of memory that represents `counter`.

So it's entirely possible for each thread to perform these tasks out of order with other threads, stomping all over each other's results. The result? You get a different answer every time.

> I would like to take a moment to praise the C++17 syntax: it looks just like Rust!