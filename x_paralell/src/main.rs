use std::thread;
//use std::rc::Rc;
//use std::time::Duration;
//use std::sync::Arc;

fn main() {
    let mut numbers_a = Vec::from_iter(1..=5);
    let mut numbers_b = Vec::from_iter(6..=10);
thread::spawn(move|| {
       numbers_a.push(42);
       println!("Numbers A: {:?}", numbers_a);
    }).join().unwrap();


    thread::spawn(move|| {
       numbers_b.push(44);
         println!("Numbers B: {:?}", numbers_b);
       
    }).join().unwrap(); 
    let x = Box::leak(Box::new(42));
    dbg!("X: {}", &x);
    
    println!("{}", &x);
}

