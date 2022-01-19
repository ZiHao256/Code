use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    expensive_closure: T,
    value: Option<HashMap<u32, u32>>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(c: T) -> Cacher<T> {
        Cacher {
            expensive_closure: c,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match &mut self.value {
            Some(v) => match v.get(&arg) {
                Some(&val) => val,
                None => {
                    let val = (self.expensive_closure)(arg);
                    v.insert(arg, val);
                    val
                }
            },
            None => {
                let val = (self.expensive_closure)(arg);
                let mut hash:HashMap<u32, u32> = HashMap::new();
                hash.insert(arg, val);
                self.value = Some(hash);
                val
            }
        }
    }
}

fn main() {
    generate_work_out(22, 22);
}

fn generate_work_out(intensity: u32, random_number: u32) {
    let mut cacher = Cacher::new(|num| {
        println!("wait a second ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(2));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}
