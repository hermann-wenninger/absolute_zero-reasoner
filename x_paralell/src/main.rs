use std::thread;
//use std::time::Duration;

fn main() {
    let numbers = Vec::from_iter(1..=500);
thread::scope(|s| {
    s.spawn(|| {
        println!("Länge: {}", numbers.len()); 
    }); 


    s.spawn(|| {
       numbers.push(44);
        //is numbers a reference or a copy?
        //how can i test it?
        //thread::sleep(Duration::from_secs(1));
    }); 
});
}

