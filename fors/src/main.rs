fn main() {
    let ary = [1,3,5,6,7];
    for element in ary.iter(){
        println!("{}", element);
    }

    for element in (1..10).rev(){
        println!("{}",element);
    }
}
