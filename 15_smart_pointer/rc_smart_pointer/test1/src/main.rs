
use std::rc::Rc;

enum List{
    Cons(i32, Rc<List>),
    Nil
}

use List::{Cons, Nil};

fn main() {


    let list = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(Cons(7, Rc::new(Nil)))))));

    println!("{}", Rc::strong_count(&list));

    let l1 = Cons(1, Rc::clone(&list));

    println!("{}", Rc::strong_count(&list));

    let l2 = Cons(2, Rc::clone(&list));

    println!("{}", Rc::strong_count(&list));
}
