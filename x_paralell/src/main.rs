use std::thread;
use std::time::Duration;


fn doit(){
    let id = thread::current().id();
    println!("thread id: {:?}", id);
   
}
fn main() {
    let t1 = thread::spawn(doit);
    let t2 = thread::spawn(doit);
    let t3 = thread::spawn(doit);

    // Simulate some work in the main thread
    thread::sleep(Duration::from_secs(2));
    let id = thread::current().id();
    println!("main thread id: {:?}", id);

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
