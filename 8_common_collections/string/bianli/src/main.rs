fn main() {
    for i in "नमस्ते".chars(){
        println!("{}", i);
    }

    for i in "नमस्ते".bytes(){
        println!("{}", i);
    }

    let s = String::from("नमस्ते");

    for i in (&s).chars(){
        println!("{}",i);
    }

    dbg!(&s);
}
