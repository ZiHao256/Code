pub trait Messenger{
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
where T: Messenger{
    messenger: &'a T,
    max: usize,
    value: usize
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T>{
        LimitTracker{
            messenger,
            max,
            value: 0
        }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;
        let percent = self.max as f64 /self.value as f64;

        if percent>= 1.0{
            self.messenger.send(">=100%");
        }else if percent >= 0.9{
            self.messenger.send(">=90%");
        }else if percent >= 0.75{
            self.messenger.send(">= 75%");
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger{
        send_messenger: RefCell<Vec<String>>
    }

    impl MockMessenger{
        fn new() -> MockMessenger{
            MockMessenger{
                send_messenger: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, msg: &str){
            self.send_messenger.borrow_mut().push(String::from(msg));
        }
    }



    #[test]
    fn test_mock() {
        let mock_messenger = MockMessenger::new();

        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    
        limit_tracker.set_value(100);

        assert_eq!(1, mock_messenger.send_messenger.borrow().len());
        
    
    }
}


fn main() {
    println!("Hello, world!");
}
