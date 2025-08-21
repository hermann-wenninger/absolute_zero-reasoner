use std::thread;
use std::time::Duration;



   

fn main() {
    let numbers = vec![1, 2, 3, 4, 5,6, 7, 8, 9, 10,11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut odd = vec![];
    let mut even = vec![];
    thread::spawn(move||{
        for n in numbers{
            if n % 2 == 0 {
                even.push(n);
            } else {
                odd.push(n);
            }
        }
    println!("Odd numbers: {:?}", &odd);
    println!("Even numbers: {:?}", &even);
    })
    .join()
    .expect("Thread panicked");
   
    thread::sleep(Duration::from_secs(1));
    println!("Main thread finished.");
    // This is a simple example of using threads to process a vector of numbers.
    // The main thread creates a new thread to separate odd and even numbers.
    // The results are printed after the thread completes its execution.
}
// This code demonstrates basic thread usage in Rust, including spawning a thread and waiting for it to
