fn main() {
    // 1 define a vector
    let mut v1:Vec<u32> = Vec::new();

    // 2 define
    let mut v2 = vec![1, 2, 3, 4];

    v1.push(v2[3]);
    v1.push(v2[2]);
    v1.push(v2[1]);
    v1.push(v2[0]);

    println!("v2[0] {}",v2[0]);

    println!("v1.get(1) {}",match v1.get(1){
        Some(&num) => num,
        None => 0
    });

    // 遍历
    for i in &mut v1{
        *i *= *i;
        println!("{}", i);
    }
    

}
