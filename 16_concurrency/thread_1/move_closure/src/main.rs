use std::thread;

fn main() {
    let mut v1 = vec![1,2];

    let handle = thread::spawn(move||{
        println!("{:?}", v1);
    });

    handle.join().unwrap();
}
