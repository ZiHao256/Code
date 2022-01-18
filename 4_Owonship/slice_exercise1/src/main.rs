fn main() {
    let s = String::from("hello world");

    println!("{}", first_word(&s[..]));

    println!("{}", first_word(&s));

    println!("{}", first_word("hello world"));

    println!("{}", first_word(&"hello world"[..]));
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..index];
        }
    }
    return &s[..];
}