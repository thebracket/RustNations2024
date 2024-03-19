# "Block on", by Aerosmith

So what does `#[tokio::main]` actually do?

Here's a version that doesn't use the helpful macro:

```rust,edition2021
async fn hello() {
    println!("Hello");
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(hello());
}
```

The `#[tokio::main]` macro is just a helper that lets you have an async `main` function. In this case, we've called `new_current_thread` to make a single-threaded Tokio runtime. That thread will now run until the async system terminates.

## Multiple Tokios

If you really want to, you can have a Tokio per thread!

```rust,edition2021
async fn hello() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Hello");
}

fn spawn() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(hello());
}

fn main() {
    let mut handles = Vec::new();
    for _ in 0 .. 4 {
        handles.push(std::thread::spawn(|| spawn()));
    }
    for h in handles {
        h.join().unwrap();
    }
    
}
```