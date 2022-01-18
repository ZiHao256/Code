struct Color(u32, u32, u32);
struct Point(u32, u32, u32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{} {}",black.0, origin.0);
}
