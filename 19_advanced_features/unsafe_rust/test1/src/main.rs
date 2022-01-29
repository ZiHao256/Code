fn main() {
    let mut num = 10;

    let p1 = &num as *const i32;
    let p2 = &mut num as *mut i32;

    unsafe{
        println!("{}",*p1);
        *p2 = 5;
    }
    
    println!("{}",num);

}
