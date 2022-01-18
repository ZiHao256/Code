use std::fmt::Display;

fn main() {
    let s1 = String::from("hello");

    let result: &str;

    {
        let s2 = String::from("world!");

        result = longest(&s1[..], &s2[..], "");

        println!("the longest str is :{}", result);
    }
}

fn longest<'a, T: Display>(s1: &'a str, s2: &'a str, anno: T) -> &'a str {
    println!("{}", anno);

    if s1.len() != s2.len() {
        s1
    } else { 
        s2
    }
}
