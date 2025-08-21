use std::thread;
//use std::rc::Rc;
//use std::time::Duration;
use std::sync::Arc;

fn main() {
    let mut numbers_a = Arc::new(Vec::from_iter(1..=5));
    let mut numbers_b = numbers_a.clone();

   //create two threads in a for loop
    for i in 0..2 {
        let numbers = if i == 0 { &numbers_a } else { &numbers_b };
        let numbers = Arc::clone(numbers);
        
        thread::spawn(move || {
            for number in numbers.iter() {
                println!("Thread {}: {}", i, number);
                //thread::sleep(Duration::from_millis(100));
            }
        });
    }

    // Wait for threads to finish
    thread::sleep(std::time::Duration::from_secs(1));
    println!("All threads completed.");

}

