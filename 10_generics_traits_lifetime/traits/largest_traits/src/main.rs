fn main() {
    let v1: Vec<u32> = vec![1, 2, 3, 4, 0];

    let v2 = vec!['a', 'b', 'c'];

    println!("{} {}", get_largest(&v1), get_largest(&v2));


}
// 1
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// 2

fn get_largest<T: std::cmp::PartialOrd>(v: &[T]) -> &T {
    let mut largest = &v[0];

    for i in v.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

