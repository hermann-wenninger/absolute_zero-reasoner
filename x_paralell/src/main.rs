use std::time::Duration;
use std::thread;
use std::sync::{Mutex,Arc}; // Arc muss importiert werden

fn main() {
   let m = Arc::new(Mutex::new(0));
   let n = Mutex::new(0);
   thread::scope(|s|{
    for _ in 1..=10{
        s.spawn(||{
            let mut guard = n.lock().unwrap();
            let mut arc_guard = m.lock().unwrap();
            *arc_guard +=1;
            *guard +=10;
            println!("Thraed hat arc_gard auf {} gesetzt", *arc_guard);
            println!("Thread hat gaerd auf {} gesetzt",  *guard);
            thread::sleep(Duration::from_millis(1));
            drop(guard);
            drop(arc_guard);
        }
     
    );
       
    }
   })
}