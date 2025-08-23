use std::time::Duration;
use std::thread::{self};
use std::sync::Mutex;
fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                    println!("Zwischenschritt: {}", *guard);
                }
                drop(guard);
                thread::sleep(Duration::from_millis(100)); // New!
            });
        }
    });
    //assert_eq!(n.into_inner().unwrap(), 1000);
    println!("Anzahl: {}", n.into_inner().unwrap());
    println!("Fertig!");
}