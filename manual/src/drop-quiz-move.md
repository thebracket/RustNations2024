# Pop Quiz

Who actually knows what `std::move` in C++, and regular moving in Rust *does*?

```rust
struct MyStruct;

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

fn move_me(a: MyStruct) {
    println!("Entered function");
}

fn main() {
    println!("Started");
    let a = MyStruct;
    move_me(a);
    println!("Exited Function");
}
```

Can you guess the output?







```
Started
Entered function
Bye!
Exited Function
```

`Drop` fires after `a` leaves the function. It was *moved* into the function, and not moved back.

In C++, when you `std::move`:

* The destructor fires.
* A "move constructor" (syntax `X(X&& other)`) fires and copies the data.
* The variable is converted to an `xvalue`---it's in a valid but undefined state. You may be able to use it, you may not!

Rust also uses an `xvalue`, but prohibits you from using the "moved from" data. That's Rust saving you from another common bug!