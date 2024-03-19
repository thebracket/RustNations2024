# Add the other REST verbs

Returning to our server project, we can add the other REST verbs. Let's start with a single book. We'd like to obtain the ID number from the path. Axum supports path extraction as follows:

```rust
let app = axum::Router::new()
    .route("/hello", get(say_hello))
    .route("/", get(get_blog_posts_handler))
    .route("/:id", get(get_blog_post_handler))
    .layer(Extension(pool.clone()));
```

Notice that you *label* the portion to extract with `:id`.

And in the handler:

```rust
async fn get_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> axum::Json<BlogPost> {
    let post = get_blog_post(pool, id).await.unwrap();
    axum::Json(post)
}
```

The `axum::extract::Path(id)` follows the same rules as an extension layer---Axum will inject the value for you. So now you just need to call your database and let Axum handle the JSON.

Run the server and go to `http://localhost:3001/1`. You'll see a single post.

## Adding Posts

We want to handle POST requests to add blog entries. In our router:

```rust
// Build Axum Router and run it
use axum::routing::{get, post};
let app = axum::Router::new()
    .route("/hello", get(say_hello))
    .route("/", get(get_blog_posts_handler))
    .route("/:id", get(get_blog_post_handler))
    .route("/add", post(add_blog_post_handler))
    .layer(Extension(pool.clone()));
```

And our handler is another thin wrapper over the database:

```rust
async fn add_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Json(post): axum::extract::Json<BlogPost>,
) -> axum::Json<i32> {
    let id = add_blog_post(pool, post.date, post.title, post.body, post.author).await.unwrap();
    axum::Json(id)
}
```

## Updating Posts

Update the router to both be a POST, and specify the ID number in the 

```rust
.route("/update/:id", post(update_blog_post_handler))
```

```rust
async fn update_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::extract::Json(post): axum::extract::Json<BlogPost>,
) -> axum::Json<()> {
    update_blog_post(pool, id, post.date, post.title, post.body, post.author).await.unwrap();
    axum::Json(())
}
```

## Deleting Posts

The router: 

```rust
.route("/delete/:id", post(delete_blog_post_handler))
```

The handler:

```rust
async fn delete_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> axum::Json<()> {
    delete_blog_post(pool, id).await.unwrap();
    axum::Json(())
}
```