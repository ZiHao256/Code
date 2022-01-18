struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &(self.x)
    }
}

impl Point<f32, f32> {
    fn y(&self) -> f32 {
        self.y
    }
}

fn main() {
    let a = Point { x: 10, y: 10.0 };
    dbg!(a.x());

    let b = Point {
        x: 1.0 as f32,
        y: 2.0 as f32,
    };

    dbg!(b.y());
}
