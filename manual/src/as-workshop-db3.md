# Validate The Data

SQLX is not an Object-Relational-Mapper (if you want one of those, Diesel and SeaOrm are pretty good). But it does have some helpers that make life easier!

Add the following to your code:

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}
```

`FromRow` lets you map a database query results to a structure, without having to painstakingly gather all the columns by hand (you can still do that if you prefer!).

So let's add a function to read the empty `blog_post` table (and make sure it exists):

```rust
async fn get_blog_posts(pool: sqlx::SqlitePool) -> Result<Vec<BlogPost>> {
    let posts = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&pool)
        .await?;
    Ok(posts)
}
```

And call it in `main`:

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

    println!("{:?}", get_blog_posts(pool).await?);

    Ok(())
}
```

If nothing went wrong, you'll see `[]` emitted. The database and table exist! (You'll get an error message if something did go wrong)

## Let's Add Some Data

Let's add a second migration:

```bash
sqlx migrate add data
```

And open up the new `.sql` file in `migrations/` and add:

```sql
INSERT INTO blog_posts (date, title, body, author) VALUES
    ('2021-01-01', 'A Tale of Two Cities', 'It was the best of times, it was the worst of times.', 'Dickens'),
    ('2021-01-02', 'Moby Dick', 'Call me Ishmael.', 'Melville');
```

> You can add whatever you want. I really don't mind!

Now run your program again, and you should see your newly added posts. (If you don't, you might need to run `cargo clean` in case the macro didn't pick up the changes properly)

```
Connecting to: sqlite://blog.db
Running migrations
[BlogPost { id: 1, date: "2021-01-01", title: "A Tale of Two Cities", body: "It was the best of times, it was the worst of times.", author: "Dickens" }, BlogPost { id: 2, date: "2021-01-02", title: "Moby Dick", body: "Call me Ishmael.", author: "Melville" }]
```

*Migrations adds a table to your database to track which migrations have been run. SQLX will avoid running the same migration twice.*