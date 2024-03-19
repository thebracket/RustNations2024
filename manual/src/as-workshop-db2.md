# Add a Database

Now that you have a connection string and the dependencies, lets create a database.

1. In a command prompt, ensure that you are in the top-level directory of your project.
2. Create the databsase with `sqlx database create`.
    * Notice that `blog.db` now exists!
2. Run `sqlx migrate add initial`

You'll see that a `migrations` directory has been created, with a timestamped migration. Let's open that file:

```sql
-- Add migration script here
```

Helpful! Now let's paste in some actual SQL:

```sql
CREATE TABLE blog_posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    title TEXT,
    body TEXT,
    author TEXT
);
```

> You can run your migrations with `sqlx migrate run`, but we're going to do it in Rust!

## Setup Rust

Let's add a function to obtain a connection pool for our databsase:

```rust
async fn get_connection_pool(url: &str) -> Result<sqlx::SqlitePool> {
    let connection_pool = sqlx::SqlitePool::connect(url)
        .await?;
    Ok(connection_pool)
}
```

And a second function that uses SQLX's `migrate` macro to run our migration:

```rust
async fn run_migrations(pool: sqlx::SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    Ok(())
}
```

Finally, let's update our `main` function to use this:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Read the .env file and apply it
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    println!("Connecting to: {database_url}");

    // Setup the database
    let pool = get_connection_pool(&database_url).await?;
    println!("Running migrations");
    run_migrations(pool.clone()).await?;
}
```

Run the program. You should see `blog.db` also gain some shared memory, and write-ahead log files. That implies it did something!