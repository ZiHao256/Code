#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shoes() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

fn shoes_in_my_size(shoes: Vec<Shoe>, mysize: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == mysize).collect()
}
