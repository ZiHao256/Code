use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters {
    value: i32,
}

struct KiloMeters{
    value:i32
}

impl Add<KiloMeters> for Meters {
    type Output = Self;

    fn add(self, other: KiloMeters) -> Self {
        Meters{
            value: self.value + other.value * 1000
        }
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self{
            value:self.value+self.value
        }
    }
}

fn main() {
    let m = Meters{value: 10};

    let km = KiloMeters{value: 1};

    let m = m+km;

    dbg!(m);

    let m = m + m;

    dbg!(m);
}
