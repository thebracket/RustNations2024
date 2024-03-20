#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        println!("Listening for mail...");
        while let Some(msg) = rx.recv().await {
            println!("{msg} from async");
        }
    });

    std::thread::spawn(move || {
        for i in 0..10 {
            tx.blocking_send(i).unwrap();
            std::thread::sleep(std::time::Duration::from_secs_f32(0.01));
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}