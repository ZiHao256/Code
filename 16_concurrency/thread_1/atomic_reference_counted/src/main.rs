use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let mut handles = vec![];
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = (*counter).lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles{
        h.join().unwrap();
    }

    println!("res: {}", *(counter).lock().unwrap())
}
