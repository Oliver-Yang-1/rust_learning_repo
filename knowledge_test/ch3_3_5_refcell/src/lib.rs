use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub fn increase_shared_value(value: &Rc<RefCell<i32>>, amount: i32) -> i32 {
    *value.borrow_mut() += amount;
    *value.borrow()
}

#[cfg(test)]
mod tests {
    use super::{LimitTracker, Messenger, increase_shared_value};
    use std::cell::RefCell;
    use std::rc::Rc;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn sends_warning_message_when_over_75_percent() {
        let messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&messenger, 100);

        tracker.set_value(80);

        assert_eq!(messenger.sent_messages.borrow().len(), 1);
        assert_eq!(
            messenger.sent_messages.borrow()[0],
            "Warning: You've used up over 75% of your quota!"
        );
    }

    #[test]
    fn rc_refcell_allows_shared_mutation() {
        let shared = Rc::new(RefCell::new(5));
        let alias_a = Rc::clone(&shared);
        let alias_b = Rc::clone(&shared);

        assert_eq!(increase_shared_value(&alias_a, 10), 15);
        assert_eq!(*alias_b.borrow(), 15);
    }
}
