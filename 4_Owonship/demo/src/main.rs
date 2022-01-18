fn main() {
    let mut str = String::from("hello");
    str.push_str(" world");

    let len = calculate_len(&mut str);

    println!("{} {}",str,len);
}


fn calculate_len(s:&mut String) -> usize{
    s.push_str(", hello world!");
    let len = s.len();
    len
}