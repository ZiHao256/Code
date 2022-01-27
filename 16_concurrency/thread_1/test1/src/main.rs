use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(||{
        for i in 0..10{
            println!("from spawn thread: {}", i);
            thread::sleep(Duration::from_millis(1)) ;
        }
    });

    for i in 0..=5{
        println!("from main thread: {}",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
