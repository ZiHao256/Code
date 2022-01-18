fn main() {
    let mut s1 = String::from("hello");
    
    println!("{} {}",calculate_length(&mut s1),s1);

    let s2 = &mut s1;

    let s3 = &s1;
    println!("{}",s1);

}

fn calculate_length(s: &mut String) -> usize{
    s.push_str(" world!");
    s.len()
}