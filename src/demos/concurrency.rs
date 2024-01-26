use std::time::Duration;

use rand::Rng;
use tokio::sync::broadcast;

#[tokio::main]
pub async fn demo_hello_tokio() {
    println!("Let's try something concurrent!");

    let (tx, mut rx) = broadcast::channel(128);

    tokio::spawn(async move {
        loop {
            // tokio::time::sleep(Duration::from_millis(314)).await;
            match rx.recv().await {
                Ok(str) => println!("Received: {str}"),
                Err(_) => println!("Oh no, something died here!")
            }
        }
    });

    let mut rng = rand::thread_rng();
    let content = ["hello", "darkness", "my", "old", "friend", "I've", "come", "to talk", "with you", "again"];

    for token in content {
        let sleep_duration = rng.gen_range(100..600);
        tx.send(String::from(token)).unwrap();
        let message = format!("// gonna nap for {} ms", sleep_duration).to_string();
        match tx.send(message) {
            Ok(_) => {}
            Err(_) => println!("Message sending failed!")
        }
        tokio::time::sleep(Duration::from_millis(sleep_duration)).await;
    }
}