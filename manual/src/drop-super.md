# So WHY is this Super?

When you're writing Rust, have you ever noticed...

* That when you open a `File`, you don't have to remember to flush and close it? Rust's `File` does that for you on `Drop`. In C, it's not uncommon to see `fflush(fd); fsync(fd); fclose(fd)` - and now `fd` is an invalid file descriptor, your file is on the disk.
* Ever used an MPSC (Multi-Producer, Single Consumer) channel to communicate between threads, and noticed that you *didn't* have to explicitly close it? Channel receivers and transmitters use `Drop` to close the channel when there are no more transmitters --- and notify the receiver. Gophers have plenty of "fun" with this one!
* When you called `let lock = my_mutex.lock()`, you didn't have to remember to call `unlock`? Again, `Drop` fires and relinquishes the lock for you.

That's a super-power in and of itself: it's really hard to forget to clean up a resource when the cleanup happens automatically.

## Even Better - Reliable Garbage Collection with Completion!

Let's pull some of what we've just covered together:

```rust
use std::sync::Mutex;

struct MyBusinessObject {
    calc1: Mutex<Option<i32>>,
    calc2: Mutex<Option<i32>>,
}

impl MyBusinessObject {
    fn new() -> Self {
        Self { calc1: Mutex::new(None), calc2: Mutex::new(None) }
    }

    fn calc1(&self) {
        // Pretend to do a real calculation
        let mut i = 0;
        for _ in 0 .. 100_000 {
            i += 1;
        }
        let mut lock = self.calc1.lock().unwrap();
        *lock = Some(i);
    }

    fn calc2(&self) {
        // Pretend to do a real calculation
        let mut i = 1_000_000;
        for _ in 0 .. 100_000 {
            i -= 1;
        }
        let mut lock = self.calc2.lock().unwrap();
        *lock = Some(i);
    }
}

impl Drop for MyBusinessObject {
    fn drop(&mut self) {
        let one = self.calc1.lock().unwrap();
        let two = self.calc2.lock().unwrap();
        println!("{}, {}", one.unwrap(), two.unwrap());
    }
}

fn main() {
    let data = MyBusinessObject::new();
    std::thread::scope(|scope| {
        scope.spawn(|| data.calc1());
        scope.spawn(|| data.calc2());
    })
}
```

This example:

* Creates a structure with interior mutability.
* Sets up some calculations.
* Sets up `Drop` to ensure that when it is done - the results are output. This could easily be saved to disk, database, or sent somewhere.
* Then in `main`, we launch two threads that independently perform the calculation.
* We're using scoped threads, so no need to `join` or wait for them.
* When they finish---in whatever order, but not before---the variable will drop and the completion runs.

Now imagine wrapping your business object in an `Arc` and passing it around your application - safe in the knowledge that it'll do the right thing when it finishes. (Hint: you might want to include some error handling!)

This is *exactly* what happens with `sqlx` database connections and connection pools, network connections, allocated memory. You can put it to use in your applications.

## Drop Is Not Infallible

Before you get too excited, there is a downside...

Try this:

```rust
fn main() {
    let m = std::sync::Mutex::new(5);
    let my_lock = m.lock();
    let my_other_lock = m.lock();
}
```

When the Playground times out, you'll see the problem. You've *deadlocked*. Drop doesn't protect you from things like trying to lock the same mutex twice in the same thread.

You can clean it up with a scope:

```rust
fn main() {
    let m = std::sync::Mutex::new(5);
    {
        let my_lock = m.lock();
    }
    let my_other_lock = m.lock();
}
```

Or an explicit drop:

```rust
fn main() {
    let m = std::sync::Mutex::new(5);
    let my_lock = m.lock();
    std::mem::drop(my_lock);
    let my_other_lock = m.lock();
}
```