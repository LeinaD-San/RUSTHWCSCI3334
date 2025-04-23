/*

Assignment 1: Spawning Threads and Joining Them
Objective:

Spawn 3 threads
Each thread should print its identifier
The main thread should wait for all threads to complete
After all threads have finished, the main thread should print "All threads completed."
Starter Code:

use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 3 threads
    for i in 1..=3 {
        // TODO: Spawn a thread and store its handle
        let handle = thread::spawn(move || {
            // Simulate some work
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });
        
        // TODO: Store the handle
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    
    
    println!("All threads completed.");
}
Hints:

Look at the JoinHandle object returned by thread::spawn
The join() method on a thread handle will wait for that thread to finish
You can use a for loop to iterate through all the handles in your vector
Don't forget to use unwrap() or error handling when joining threads

*/

use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");
    
    // TODO: Create a vector to store thread handles
    let mut handles= vec![];
    
    // TODO: Spawn 3 threads
    for i in 1..=3 {
        // TODO: Spawn a thread and store its handle
        let handle = thread::spawn(move || {
            // Simulate some work
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });
        
        // TODO: Store the handle
        handles.push(handle);  
    }

    // TODO: Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("All threads completed.");
}