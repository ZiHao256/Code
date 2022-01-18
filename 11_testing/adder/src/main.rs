use adder::*;



fn main(){
    println!("hello world");
    
    let r1 = Rectangles::new(1, 1);

    let r2 = Rectangles::new(2, 2);

    println!("{}",r1.can_hold(&r2));

}