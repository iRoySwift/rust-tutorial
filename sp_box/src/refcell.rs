// RefCell<T> 和 内部可变性 interior mutability
// 内部可变性： 可变的借用一个不可变的值

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent_of_max = self.value as f64 / self.max as f64;
        if percent_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percent_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota");
        } else if percent_of_max >= 0.75 {
            self.messenger
                .send("Urgent warning: You've used up over 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    // 实现Messenger trait
    impl Messenger for MockMessenger {
        // fn send(&self, msg: &str);
        // Messenger trait中 send方法里self是不可变 所以不能使用&mut self;
        // 这边可以使用内部可变性RefCell<T> 方法
        fn send(&self, msg: &str) {
            self.send_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
    }
}
