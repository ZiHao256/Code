use std::collections::HashMap;

fn main() {
    // 1
    let mut h1 = HashMap::new();

    h1.insert("Blue", 10);
    h1.insert("Red", 20);


    // 2
    let v1 = vec!["Blue", "Red"];
    let v2 = vec![10, 20];

    let h2: HashMap<_, _> = v1.iter().zip(v2.iter()).collect();

    dbg!(&h2);

}
