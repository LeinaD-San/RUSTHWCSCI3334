/*
Assignment 2: Sharing Counter Data Between Threads
Objective:

Define a shared counter that starts from zero
Spawn 5 threads where each thread increments the counter by 1, 10 times
Use Arc and Mutex to share and safely update the counter across threads
The main thread should print the final value of the counter after all threads have completed
Starter Code:

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: Create a shared counter using Arc and Mutex
    
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 5 threads
    for i in 1..=5 {
        // TODO: Clone the Arc for the thread
        
        
        // TODO: Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            // TODO: Increment counter 10 times
            
            
        });
        
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    
    
    // TODO: Print the final value of the counter
    
}
Hints:

Start by initializing your counter with let counter = Arc::new(Mutex::new(0));
Remember to clone the Arc before moving it into each thread with Arc::clone(&counter)
Inside each thread, you'll need to:
Use a loop to repeat the increment 10 times
Lock the mutex with let mut num = counter_clone.lock().unwrap();
Modify the value with *num += 1;
The lock is automatically released when the MutexGuard variable goes out of scope
Don't forget to join all threads before printing the final value
To access the counter value at the end: *counter.lock().unwrap()
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: Create a shared counter using Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 5 threads
    for i in 1..=5 {
        // TODO: Clone the Arc for the thread
        let counter_clone = Arc::clone(&counter);
        
        // TODO: Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            // TODO: Increment counter 10 times
            for i in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    
    for handle in handles {
        handle.join().unwrap();
    }
    // TODO: Print the final value of the counter

    let result = *counter.lock().unwrap();
    println!("Final counter value: {}", result);
}