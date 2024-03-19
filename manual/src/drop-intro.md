# The Super-Power of Drop

> Once again giving credit where its due, C++ is the inventor of the "RAII" concept - **R**esource **A**cquisition **I**s **I**nitialization. That's probably why its such a painful acronym that neither describes it or is easy to pronounce!

RAII is everywhere. Modern C++ relies on it, Rust relies on it, the Linux kernel uses a macro-ized C version of it. It's prevalent because its such a powerful concept.

When a variable "falls out of scope"---is no longer referenced---it is "dropped". When that happens:

* Everything the structure holds is dropped.
* If the `Drop` trait is implemented, your code runs. This is just like a C++ destructor (it even uses the same mechanisms).

Here's a very simple example:

```rust
struct MyStruct;

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

fn main() {
    let a = MyStruct;
}
```

Dropping even survives panics:

```rust
struct MyStruct;

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

fn main() {
    let a = MyStruct;
    panic!("Oh no!");
}
```

> Dropping does *not* survive various UNIX signals. You need to handle `ctrl-C` yourself. You're out of luck for `kill -9`!
