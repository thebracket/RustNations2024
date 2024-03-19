# Transitive Drop

`Drop` is very, very thorough. Let's put 5 `MyStruct` into a vector, and inside a new structure that doesn't implement `Drop` at all:

```rust
struct MyStruct;

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

struct Top {
    a: MyStruct,
}

fn main() {
    let vec = vec![MyStruct, MyStruct, MyStruct, MyStruct, MyStruct];
    let top = Top { a: MyStruct };
}
```

Running this yields 6 "Bye!" statements. Every one was dropped.

> Destructors in garbage collected languages can be scary. It's unclear exactly *when* a variable will genuinely cease to exist, so it's not always clear when the destructor fires. Other languages often work around this with `using`, `with` and similar structures. Rust makes it obvious.