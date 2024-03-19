# Dropping Shared Structures

When you use `Rc`, `Arc` or a similar "shared ownership" structure you are guaranteed to only have *one* of that structure. Access is reference counted: when you `clone` the `Rc`, the reference count goes up. When it drops, the reference count goes down. When the reference count hits zero, the contents of the `Rc` and the `Rc` itself are dropped.

For example:

```rust
use std::rc::Rc;

struct MyStruct;

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

fn do_something(a: Rc<MyStruct>) {
    println!("Did something");
}

fn main() {
    let my_shared = Rc::new(MyStruct);
    for _ in 0..10 {
        do_something(my_shared.clone());
    }
}
```

Even though we've cloned our `Rc` 10 times, it only drops once.

`Rc` itself is working *because* of the power of `Drop`. You don't have to release the reference or do anything special---it just works. We don't have garbage collection, but sometimes it really feels like we do!