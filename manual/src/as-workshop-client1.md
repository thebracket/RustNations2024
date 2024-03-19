# Let's build a client

Let's create a separate project for our client:

```bash
cargo new blog_client
```

And we'll add some dependencies:

```bash
cargo add clap -F derive
cargo add serde -F derive
cargo add reqwest -F json
cargo add tokio -F full
```

Now, let's copy/paste our blog client type and setup an async main:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
```

## Clap for Command Line Handling

Let's setup a basic Clap command-line processor:

```rust
#[derive(Parser, Debug)]
#[clap(name = "blog_client", version = "1.0", author = "Your Name")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(name = "list")]
    List,
    #[clap(name = "show")]
    Show { id: i32 },
    #[clap(name = "create")]
    Create { title: String, body: String, author: String },
    #[clap(name = "delete")]
    Delete { id: i32 },
}

#[tokio::main]
async fn main() {
    let args = Opts::parse();
    match args.subcmd {
        _ => println!("Run with --help for help")
    }
}
```

Running this gives you a really nice framework, listing commands.

## Getting all blog posts

Now let's add our first subcommand and use `Reqwest`:

```rust
#[tokio::main]
async fn main() {
    let args = Opts::parse();
    match args.subcmd {
        SubCommand::List => {
            let posts = reqwest::get("http://localhost:3001/")
                .await
                .unwrap()
                .json::<Vec<BlogPost>>()
                .await
                .unwrap();
            for post in posts {
                println!("{}: {}", post.id, post.title);
            }
        }
        _ => println!("Run with --help for help")
    }
}
```

Run the program (while your server is running!) and you will see:

```
cargo run -- list
1: A Tale of Two Cities
2: Moby Dick
```