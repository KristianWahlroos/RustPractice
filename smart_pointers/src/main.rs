use List::{Cons, Nil};
use std::mem::drop;
use std::rc::Rc;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // let list = Cons(1,
    // Box::new(Cons(2,
    // Box::new(Cons(3,
    // Box::new(Nil))))));

    let jou_list = Rc::new(Cons(1,
    Rc::new(Cons(2,
    Rc::new(Cons(3,
    Rc::new(Nil)))))));

    println!("There are {} jous at the x!", Rc::strong_count(&jou_list));
    let jou_a_list = Cons(3, Rc::clone(&jou_list));
    println!("There are {} jous at the x!", Rc::strong_count(&jou_list));
    let jou_b_list = Cons(4, Rc::clone(&jou_list));
    println!("There are {} jous at the x!", Rc::strong_count(&jou_list));

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, *y);



    let m = Box::new(String::from("Rust"));
    hello(&m);
    let n = Box::new(String::from("Rust"));
    hello(&(*n)[..]);

    let e = JouDrop {data: String::from("Drop the mic!")};
    drop(e);
    let c = JouDrop {data: String::from("Drop the jou program! JOU!")};
    let d = JouDrop {data: String::from("Drop the mike!")};
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

struct JouDrop { 
    data : String,
}

impl Drop for JouDrop {
    fn drop(&mut self) {
        println!("Dropping jou jou {}", self.data);
    }
}

pub trait Joussenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Joussenger> {
    joussenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Joussenger {
    pub fn new(joussenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            joussenger,
            value: 0,
            max,
        }
    }


pub fn set_value(&mut self, value:usize) {
    self.value = value;
    let percentage_of_max = self.value as f64 / self. max as f64;
    if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
        self.joussenger.send("Warning: You did bad, you used almost all your quota");
    } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.joussenger.send("Urgent warning: Stop stupid, your quota is almost done");
    } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.joussenger.send("Error: You are done for!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockJoussenger {
        sent_messages: RefCell<Vec<String>>, 
    }
    impl MockJoussenger {
        fn new() -> MockJoussenger {
            MockJoussenger { sent_messages: RefCell::new(vec![]) }
        }
    }
    impl Joussenger for MockJoussenger {
        fn send(&self, message:&str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_send_over_75_pervent_warning_message() {
        let mock_joussenger = MockJoussenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_joussenger, 100);

        limit_tracker.set_value(89);
        assert_eq!(mock_joussenger.sent_messages.borrow().len(), 1);
    }
}