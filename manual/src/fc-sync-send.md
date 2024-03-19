# WHY does this work?

This is a layered question:

* Rust auto-implements `Sync` and `Send` for types.
    * `Send` types can be safely sent between threads. Most types are `Send`.
    * `Sync` types can be safely *immutably* borrowed between threads. Most types are `Sync`.
    * Raw pointers are neither `Sync` nor `Send` - you have to wrap them in safe types.
    * `UnsafeCell` and other types that offer non-thread-safe mutability aren't thread-safe - so they don't implement `Sync`.
    * `Rc` is explicitly neither `Sync` nor `Send` because it isn't thread-safe.

So at this layer, Rust is preventing you from sending things that aren't safe in a threaded context across thread boundaries.

At the *next* level down, the borrow checker is keeping track of access permissions to fields. You can't mutably access/borrow a target more than one at a time. That's the protection that made compiling data-races require `unsafe` code and a `static`.

So why does a `Mutex` or `AtomicX` allow it? The borrow is *immutable*. When you declared your mutex, it wasn't mutable!

```rust
let counter = Mutex::new(Counter(0));
```

Since `Mutex` is safe to immutably access across threads, the key is *interior mutability*. Mutex itself remains immutable---but it provides `Sync` friendly locking inside. This ensures that the borrow checker's mutability rules are followed.

It's very clever, and reasonably bulletproof.

## Making Use of Interior Mutability

You can make use of interior mutability yourself. Imagine a structure:

```rust
struct SomeData {
    a: String,
    b: i32,
    c: ComplicatedData,
}
```

And also say you have different threads doing different things with it.

You might be tempted to protect it as follows:

```rust
let shared = RwLock::new(SomeData);
```

We've been clever and used a read-write lock---so you can have lots of readers, and occasionally obtain a write lock to change something. That's great, but what if you have a lot of changes? Interior mutability to the rescue!

```rust
struct SomeData {
    a: Mutex<String>,
    b: AtomicI32,
    c: Mutex<ComplicatedData>,
}
```

And now each thread can access parts of the structure with minimal contention. BUT - printing the whole thing will require a bit more work.