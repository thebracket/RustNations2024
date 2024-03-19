use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Debug, Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}

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
        SubCommand::Show { id } => {
            let post = reqwest::get(&format!("http://localhost:3001/{}", id))
                .await
                .unwrap()
                .json::<BlogPost>()
                .await
                .unwrap();
            println!("{:?}", post);
        }
        SubCommand::Create { title, body, author } => {
            let post = BlogPost {
                id: 0,
                date: String::from(""),
                title,
                body,
                author,
            };
            let post = reqwest::Client::new()
                .post("http://localhost:3001/add")
                .json(&post)
                .send()
                .await
                .unwrap()
                .json::<i32>()
                .await
                .unwrap();
            println!("New Post ID: {}", post);
        }
        SubCommand::Delete { id } => {
            reqwest::Client::new()
                .delete(&format!("http://localhost:3001/delete/{}", id))
                .send()
                .await
                .unwrap();
            println!("Deleted Post ID: {}", id);
        }
        _ => println!("Run with --help for help")
    }
}
