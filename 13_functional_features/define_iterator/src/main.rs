struct Counter{
    num: u32
}

impl Counter{
    fn new() -> Counter{
        Counter{
            num:0
        }
    }
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        self.num += 1;
        if self.num <= 6{
            Some(self.num)
        }else{
            None
        }
    }
}


fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    let mut c = Counter::new();

    assert_eq!(Some(1), c.next());
}