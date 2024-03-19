# Add Other Access Functions

Let's fill out the other database access functions:

```rust
// Update your imports!
use sqlx::{FromRow, Row};
```

Get a single post:

```rust
async fn get_blog_post(pool: sqlx::SqlitePool, id: i32) -> Result<BlogPost> {
    let post = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(post)
}
```

Add a post:

```rust
async fn add_blog_post(
    pool: sqlx::SqlitePool,
    date: String,
    title: String,
    body: String,
    author: String,
) -> Result<i32> {
    let id = sqlx::query("INSERT INTO blog_posts (date, title, body, author) VALUES (?, ?, ?, ?); SELECT last_insert_rowid();")
        .bind(date)
        .bind(title)
        .bind(body)
        .bind(author)
        .fetch_one(&pool)
        .await?
        .get(0);
    Ok(id)
}
```

Update a post:

```rust
async fn update_blog_post(
    pool: sqlx::SqlitePool,
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String
) -> Result<()> {
    sqlx::query("UPDATE blog_posts SET date = ?, title = ?, body = ?, author = ? WHERE id = ?")
        .bind(date)
        .bind(title)
        .bind(body)
        .bind(author)
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}
```

Delete a post:

```rust
async fn delete_blog_post(pool: sqlx::SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM blog_posts WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}
```

And we'll extend `main` a little to do some testing:

```rust
// Test code
println!("{:?}", get_blog_posts(pool.clone()).await?);

println!("{:?}", get_blog_post(pool.clone(), 1).await?);

let new_id = add_blog_post(
    pool.clone(),
    "2021-01-01".to_string(),
    "My first blog post".to_string(),
    "This is my first blog post".to_string(),
    "Herbert".to_string()
).await?;
println!("{:?}", get_blog_post(pool.clone(), new_id).await?);

update_blog_post(
    pool.clone(),
    new_id,
    "2021-01-01".to_string(),
    "My first blog post".to_string(),
    "This is my first blog post. I have updated it.".to_string(),
    "Herbert Again".to_string()
).await?;
println!("{:?}", get_blog_post(pool.clone(), new_id).await?);

delete_blog_post(pool.clone(), new_id).await?;
println!("{:?}", get_blog_posts(pool.clone()).await?);
```