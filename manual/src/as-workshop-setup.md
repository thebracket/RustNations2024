# Setup

Let's get started by making a new project and adding Tokio to it:

```bash
cargo new webserver
cd webserver
cargo add tokio -F full
cargo add anyhow
```

> We're using the "full" feature flag so we don't have to figure out what we forgot! We're using `anyhow` to simplfy error handling.

Let's edit `main.rs` to give us a simple async "hello world":

```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
```