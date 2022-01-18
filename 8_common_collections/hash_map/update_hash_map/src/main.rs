fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 覆盖旧值
    scores.insert(String::from("Blue"), 11);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 若不存在则插入
    scores.entry("Blue".to_string()).or_insert(12);
    scores.entry("Red".to_string()).or_insert(10);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 根据旧值更新
    let text = String::from("hello world hello earth hello 宇宙");
    let mut hash2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = hash2.entry(word).or_insert(0);
        *count += 1;
    }

    for (k,v) in &hash2{
        println!("{}:{}",k,v);
    }
}
