fn main() {
    let v1 = vec![1, 2, 3];

    for (index, value) in v1.iter().enumerate(){
        println!("v1[{}]: {}", index,value);
    }
}
