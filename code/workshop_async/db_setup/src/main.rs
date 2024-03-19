use anyhow::Result;
use sqlx::{FromRow, Row};

async fn get_connection_pool(url: &str) -> Result<sqlx::SqlitePool> {
    let connection_pool = sqlx::SqlitePool::connect(url).await?;
    Ok(connection_pool)
}

async fn run_migrations(pool: sqlx::SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}

#[derive(Debug, FromRow)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}

async fn get_blog_posts(pool: sqlx::SqlitePool) -> Result<Vec<BlogPost>> {
    let posts = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&pool)
        .await?;
    Ok(posts)
}

async fn get_blog_post(pool: sqlx::SqlitePool, id: i32) -> Result<BlogPost> {
    let post = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(post)
}

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

async fn delete_blog_post(pool: sqlx::SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM blog_posts WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}

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

    Ok(())
}
