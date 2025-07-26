use std::time::Duration;

use tokio::{sync::broadcast, time::sleep};

async fn create_consumer(mut rx: broadcast::Receiver<&str>, name: &str) {
    loop {
        match rx.recv().await {
            Ok(msg) => println!("{name} received: {msg}"),
            Err(e) => {
                eprintln!("{name} error: {e:?}");
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(10);

    tokio::spawn(create_consumer(tx.subscribe(), "Consumer 1"));
    tokio::spawn(create_consumer(tx.subscribe(), "Consumer 2"));

    loop {
        tx.send("Hello to all consumers")
            .map(|_| println!("Message sent to consumers."))
            .unwrap_or_else(|e| eprintln!("Error sending message: {e:?}"));

        sleep(Duration::from_secs(2)).await;
    }
}
