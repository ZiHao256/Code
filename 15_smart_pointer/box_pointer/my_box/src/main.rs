use std::ops::Deref;

struct Mybox<T> (T);

impl<T> Mybox<T>{
    fn new(t: T) -> Mybox<T>{
        Mybox(t)
    }
}

impl<T> Deref for Mybox<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}


fn main() {
    
    // let x = 5;

    // let y = Mybox::new(x);

    // assert_eq!(5, *y);

    // let z = Box::new(x);

    // assert_eq!(5, *z);

    let s = String::from("hello");

    let s1 = Mybox::new(String::from("hello world"));

    get_str(&s1);
    

}

fn get_str(s: &str){
    println!("{}", s);
}