use std::thread;
use std::time::Duration;

fn main() {
    let numbers = Vec::from_iter(1..=500);
    let t1 = thread::spawn(move||{
        let len =numbers.len();
        let sum = numbers.iter().sum::<i32>();
        println!("Summe der Zahlen: {}", sum);
        println!("Anzahl der Zahlen: {}", len);
        let durchschnitt = sum as f64 / len as f64;
        println!("Durchschnitt: {}", durchschnitt);
        
    }).join().unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("{:?}",t1);
}

