use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List{
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

use List::{Cons, Nil};

impl List{
    fn tail(&self) -> Option<&RefCell<Rc<List>>>{
        match self{
            Cons(_, list) => Some(list),
            Nil => None
        }
    }
}

fn main() {
    let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));

    println!("a: {:?}", &a);

    let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));

    println!("b: {:?}", &b);

    if let Cons(_,list) = &*a {
        *list.borrow_mut() = Rc::clone(&b);
    }

    // overflow
    //println!("a: {:?}", &a) 

}
