# "Sending Messages into Thread Land"

You can use channels to send data from async-land to thread land. This works because `send` does not block; and if it *does* block, the receiver thread will activate---so you're only briefly pausing the Tokio runtime (still not a great idea - make sure you understand your bounds).

```rust
#[tokio::main]
async fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    std::thread::spawn(move || {
        println!("Listening for mail...");
        while let Ok(msg) = rx.recv() {
            println!("{msg} from async into thread");
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    tx.send(1);
}
```

So what happens if you use a threaded channel to go the other way?

> Run this locally, it is in `code/mixed_channels`. The playground won't like you.

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    tokio::spawn(async move {
        println!("Listening for mail...");
        while let Ok(msg) = rx.recv() {
            println!("{msg} from async");
        }
    });

    for i in 0..10 {
        tx.send(i).unwrap();
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    }
}
```

Oh dear. The output is:

```
Listening for mail...
0 from async
```

And it stops. Hard. You'll notice we cheated a little but and set "flavor" to "current_thread"---there is only one thread. A channel `receiver` *blocks* the thread altogether, waiting for a conditional variable to wake it up. That's really unfortunate, because Tokio is also using that thread to schedule the async runtime. Oops.

Aha! You say---we'll use Tokio's channel system. And that does indeed work... in this case:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        println!("Listening for mail...");
        while let Some(msg) = rx.recv().await {
            println!("{msg} from async");
        }
    });

    for i in 0..10 {
        tx.send(i).await;
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    }
}
```

BUT: we've not solved the problem. We want threads AND async! Let's try again:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        println!("Listening for mail...");
        while let Some(msg) = rx.recv().await {
            println!("{msg} from async");
        }
    });

    let thread = std::thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).await;
            tokio::time::sleep(std::time::Duration::from_secs_f32(0.01)).await;
        }
    });
}
```

Oh no! They told me that Rust async was awful! I can't call into an async channel from threaded land---and it'd be terrible overkill to make a whole new runtime! Whatever shall I do? Well, fortunately you have options:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        println!("Listening for mail...");
        while let Some(msg) = rx.recv().await {
            println!("{msg} from async");
        }
    });

    std::thread::spawn(move || {
        for i in 0..10 {
            tx.blocking_send(i).unwrap();
            std::thread::sleep(std::time::Duration::from_secs_f32(0.01));
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}
```

And there you have it: `blocking_send` is implemented on Tokio channels for just this purpose. You can have your async cake, and eat with threads.