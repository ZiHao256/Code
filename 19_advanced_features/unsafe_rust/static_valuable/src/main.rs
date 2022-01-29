static mut COUNTER: i32 = 0;

fn add_inc(inc:i32){
    unsafe { COUNTER += inc; }
}

fn main() {
    add_inc(10);

    unsafe { println!("COUNTER {}", COUNTER); }
}
