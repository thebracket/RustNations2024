# Your First JSON Route

We'd like going to `http://localhost:3001/` to return a JSON blob of all of our blog posts.

## Connection Pool as a Service

Wouldn't it be nice if all of our handlers shared a connection pool, rather than calling `get_connection_pool` over and over (and utterly defeating the point of having a pool)?

Axum uses a layer called `Tower` underneath it to provide "middleware". The simplest form of middleware is a layer that just provides some shared data. First, add this to your imports:

```rust
use axum::Extension;
```

And then extend your router to add an `Extension` as a layer:

```rust
use axum::routing::get;
let app = axum::Router::new()
    .route("/hello", get(say_hello))
    .layer(Extension(pool.clone()));
axum::serve(listener, app).await?;
```

Extensions should be designed to be cloned. Connection pools are ideal for this because they act like an `Arc` and are designed to be cloned everywhere.

## Support JSON

Axum has built-in JSON support. You just need to add `serde` to provide serialization and deserialization to your data type:

```bash
cargo add serde -F derive
```

And update your blog post type to be serializable/deserializable:

```rust
#[derive(Debug, FromRow, Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}
```

## Create the Handler and Route

Your handler function is actually pretty simple:

```rust
async fn get_blog_posts_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
) -> axum::Json<Vec<BlogPost>> {
    let posts = get_blog_posts(pool).await.unwrap();
    axum::Json(posts)
}
```

You can have Axum inject your layer by having a parameter with the syntax: `Extension(local_name): Extension<type>`.

You can return `axum::Json` and wrap any serializable data to have Axum handle the serialization for you.

Now go to [http://localhost:3001/](http://localhost:3001/) and you will see a JSON dump of your blog posts!