use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move ||{
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    thread::spawn(move||{
        let val = vec!["hello", "world"];

        for i in val{
            tx2.send(String::from(i)).unwrap();
        }
    });

    for i in rx{
       println!("{}", i); 
    }
    
}
