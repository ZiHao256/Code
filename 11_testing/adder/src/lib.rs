#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_private_func() {
        assert_eq!(internal_adder(2, 2), 4);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_can_hold() {
        let r1 = Rectangles {
            width: 30,
            height: 30,
        };
        let r2 = Rectangles {
            width: 20,
            height: 20,
        };

        assert_eq!(r1.can_hold(&r2), true, "{:?} cant hold {:?}", r1, r2);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn test_guess_new() {
        Guess::new(-1);
    }

    #[test]
    #[ignore]
    fn test_result() -> Result<(), String> {
        match 2 + 2 {
            4 => Ok(()),
            other => Err(format!("anwser is {}", other)),
        }
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
pub struct Rectangles {
    width: u32,
    height: u32,
}

impl Rectangles {
    pub fn can_hold(&self, another: &Rectangles) -> bool {
        self.width >= another.width && self.height >= another.height
    }

    pub fn new(width:u32, height:u32) -> Rectangles{
        Rectangles{
            width,
            height
        }
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}
