use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::thread_rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;
    
    let (tx, rx) = mpsc::channel();
    let shared_rx = Arc::new(Mutex::new(rx));
    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];
    for i in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(i + 1, tx_clone, ITEM_COUNT / NUM_PRODUCERS);
        });
        producer_handles.push(handle);
    }
    for i in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&shared_rx);
        let handle = thread::spawn(move || {
            consumer(i + 1, rx_clone);
        });
        consumer_handles.push(handle);
    }
    for handle in producer_handles {
        handle.join().unwrap();
    }
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    drop(tx);
    for handle in consumer_handles {
        handle.join().unwrap();
    }
    println!("All items have been produced and consumed!");
}
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut Rng = thread_rng();
    for _ in 0..item_count {
        let val = Rng.gen_range(1..100);
        println!("Producer {}: sending {}", id, val);
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished.", id);
}
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let message = {
            let guard = rx.lock().unwrap();
            guard.recv()
        };
        match message {
            Ok(TERMINATION_SIGNAL) => {
                println!("Consumer {}: received termination signal. Now Exiting.", id);
                break;
            }
            Ok(val) => {
                println!("Consumer {}: processed value {}", id, val);
                thread::sleep(Duration::from_millis(150));
            }
            Err(_) => {
                println!("Consumer {}: channel closed. Now Exiting.", id);
                break;
            }
        }
    }
}