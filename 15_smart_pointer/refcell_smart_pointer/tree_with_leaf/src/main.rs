use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    Children: RefCell<Vec<Rc<Node>>>,
    Parent: RefCell<Weak<Node>>
}

fn main() {
    let leaf = Rc::new(Node {
        value: 1,
        Children: RefCell::new(vec![]),
        Parent: RefCell::new(Weak::new())
    });

    let node = Rc::new(Node {
        value: 2,
        Children: RefCell::new(vec![Rc::clone(&leaf)]),
        Parent: RefCell::new(Weak::new())
    });

    *leaf.Parent.borrow_mut() = Rc::downgrade(&node);

    println!("{:?}", (*leaf).Parent.borrow().upgrade());
}
