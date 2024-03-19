# Hello Web

Let's add `Axum` --- our webserver --- to the project:

```bash
cargo add axum
```

And let's add a second environment variable to `.env`:

```
DATABASE_URL="sqlite://blog.db"
LISTEN_ADDRESS="0.0.0:3001"
```

Now delete the database test code from `main` and replace it with:

```rust
// TCP Listener
let listen_address = std::env::var("LISTEN_ADDRESS")?;
println!("Listening on: {listen_address}");
let listener = tokio::net::TcpListener::bind(&listen_address).await?;
```

We get the environment variable, print it. Then we create a `TcpListener` which opens the requested TCP port, and listens for connections.

Now lets create an `Axum` application router. This is a lot like other webservers, you are mapping routes to handler functions:

```rust
// Build Axum Router and run it
use axum::routing::get;
let app = axum::Router::new()
    .route("/hello", get(say_hello));
axum::serve(listener, app).await?;
```

Finally, `say_hello` needs to be defined:

```rust
async fn say_hello() -> &'static str {
    "Hello, World!"
}
```

> We're just returning a string, nothing fancy.

Now you can run `cargo run` and point a browser at [http://localhost:3001/hello](http://localhost:3001/hello).

You now have Axum running!