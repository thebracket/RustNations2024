#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    tokio::spawn(async move {
        println!("Listening for mail...");
        while let Ok(msg) = rx.recv() {
            println!("{msg} from async");
        }
    });

    for i in 0..10 {
        tx.send(i).unwrap();
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    }
}
