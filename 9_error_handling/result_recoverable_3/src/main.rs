use std::fs::File;

fn main() {
    // method: unwrap
    // dbg!("unwrap\n");
    // let mut f = File::open("hello.txt").unwrap(); 

    // method: except
    dbg!("except\n");
    let mut f = File::open("hello.txt").expect("open failed");

}
