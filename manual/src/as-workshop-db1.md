# Add a Database

We're going to use SQLX. So let's add it as a dependency. The feature flags for SQLX are fun, and I find myself checking the documentation every time!

Install the dependency with:

```bash
cargo add sqlx -F runtime-tokio-rustls -F sqlite -F chrono
```

And the helper CLI tool (which will also install SQLite on most versions):

```bash
cargo install sqlx-cli
```

## Connection String

Let's pretend that we're working in a Kubernetes monster or other dynamic environment. We'll obtain the database URL from an environment variable. But typing `DATABASE_URL="sqlite://" cargo run` every time is a pain, so we'll also use `dotenvy` to allow us to load our connection strings from a file named `.env`.

Install the dependency:

```bash
cargo add dotenvy
```

And lets make `main.rs` read it:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Read the .env file and apply it
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    println!("Connecting to: {database_url}");
    Ok(())
}
```

This will fail if you run it with `cargo run`, and work if you `DATABASE_URL="sqlite://" cargo run` (or equivalent on Windows). Now let's create a file named `.env`. It goes in your top-level project folder, next to `src` and `Cargo.toml`

```
DATABASE_URL="sqlite://blog.db"
```