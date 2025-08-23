use std::time::Duration;
use std::thread;
use std::sync::{Mutex}; // Arc muss importiert werden

fn main() {
   let n = Mutex::new(0);
   thread::scope(|s|{
    for _ in 1..=10{
        s.spawn(||{
            let mut guard = n.lock().unwrap();
            *guard +=10;
            println!("Thread hat gaerd auf {} gesetzt",  *guard);
            thread::sleep(Duration::from_millis(50));
        });
    }
   })
}