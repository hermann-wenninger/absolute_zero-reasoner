use std::thread;
//use std::time::Duration;

fn main() {
    let numbers = Vec::from_iter(1..=500);
thread::scope(|s| {
    s.spawn(|| {
        println!("Länge: {}", numbers.len()); 
    }); 

    
    s.spawn(|| {
        for n in &numbers { 
            println!("{n}"); 
        } 
    }); 
});
}

