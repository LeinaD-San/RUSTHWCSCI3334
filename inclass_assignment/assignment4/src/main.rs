/*
Assignment 4: Producer-Consumer Pattern with Termination Signal
Objective:

Implement a producer-consumer pattern using a single channel
Create 2 producer threads that generate random numbers and send them to a channel
Create 3 consumer threads that receive numbers from the channel and process them
After producing the required number of items, send a special termination value to signal consumers to exit
Each thread should identify itself when printing results
Use proper synchronization to ensure clean shutdown
Starter Code:

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    
    
    // TODO: Create 2 producer threads
    
    
    // TODO: Create 3 consumer threads
    
    
    // TODO: Wait for all threads to finish
    
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
}
Hints:

Create a single mpsc channel for data transfer: let (tx, rx) = mpsc::channel();
Use Arc<Mutex<Receiver<i32>>> to share the receiver among consumer threads
After producers have sent all their data items, the main thread should:
Send the termination signal (TERMINATION_SIGNAL) once for each consumer
Example: for _ in 0..num_consumers { tx.send(TERMINATION_SIGNAL).unwrap(); }
In the consumer function:
Check if received value equals TERMINATION_SIGNAL
If it does, break the loop and exit the thread
If not, process the value normally
Make sure to wait for all threads to complete before exiting the program
*/
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx,rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    // TODO: Create 2 producer threads
    let mut producer_handles = vec![];
    let items_per_production = ITEM_COUNT / 2;
    for id in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(id, tx_clone, items_per_production);
        });
        producer_handles.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    let mut consumer_handles = vec![];
    for id in 0..3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        consumer_handles.push(handle);
    }
    
    // TODO: Wait for all threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    for handle in consumer_handles {
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();
    for i in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {} sending number: {}",id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished sending {} items", id, item_count);
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let num = rx.lock().unwrap().recv().unwrap();
        if num == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal",id);
            break;
        }
        println!("Conumer {} received number: {}",id, num);
        thread::sleep(Duration::from_millis(50));
    }
    println!("Consumer {} shutting down", id);
}