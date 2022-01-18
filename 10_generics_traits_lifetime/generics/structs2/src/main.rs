// 获取另一个 Point 作为参数，
// 而它可能与调用 mixup 的 self 是不同的 Point 类型
#[derive(Debug)]
struct Point<T,U>{
    x:T,
    y:U
}

impl<T,U> Point<T,U>{
    fn exchange<W,V>(self, other: Point<W,V>) -> Point<T,V>{
        Point{
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let p1 = Point{x:1, y:2};
    let p2 = Point{x:"xx", y:"yy"};

    let p3 = p1.exchange(p2);

    dbg!(p3);

}
