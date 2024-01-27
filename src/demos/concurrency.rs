use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use rand::Rng;
use tokio::sync::broadcast;
use std::thread;
use std::thread::sleep;

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

// Slightly adapted from https://www.thorsten-hans.com/weekly-rust-trivia-share-state-between-threads/
pub fn demo_threading() {
    let state = Arc::new(Mutex::new(vec![]));

    let consumer_data = Arc::clone(&state);
    let spawned_consumer = thread::spawn(move || {
        println!("Giving producers some head start");
        sleep(Duration::from_millis(250));
        match consumer_data.lock() {
            Err(_) => println!("Oops, failed to access the data"),
            Ok(data) => {
                println!("Almost there...");
                for entry in data.to_vec() {
                    println!("Consumer is happy with {entry}");
                }
            }
        }
        println!("Done consuming!");
    });

    let producer_data = Arc::clone(&state);
    let spawned_producer = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        println!("Producing stuff before the consumer wakes up!");
        for _ in 1..7 {
            match producer_data.lock() {
                Err(_) => println!("Producer could not access shared state"),
                Ok(mut data) => {
                    let ms_since_epoch = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                    data.push(format!("It has been so many {ms_since_epoch} ms since epoch...").to_string());
                }
            }
            sleep(Duration::from_millis(rng.gen_range(50..90)))
        }
        println!("Produced all I could, I am depleted");
    });

    spawned_producer.join().unwrap();
    spawned_consumer.join().unwrap();

    let final_state = state.lock().unwrap().to_vec();
    println!("The drill is over, everyone go home, thank you. By the way, the state ends up as {final_state:?}");
}