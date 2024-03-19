# Finishing the client

All we have to do to finish the client is paste in the remaining verbs:

```rust
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
```