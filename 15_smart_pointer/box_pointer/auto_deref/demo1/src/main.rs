use std::rc::Rc;
use std::ops::Deref;

fn main() {
    let s = Rc::new(String::from("hello"));

    let s1 = s.deref();

    let s2 = s.deref().deref();

    println!("{}",s.len());
    println!("{}",s1.len());
    println!("{}",s2.len());

    let x1 = (*s).len();

    let x2 = &*s;

    let x3 = &**s;
}
