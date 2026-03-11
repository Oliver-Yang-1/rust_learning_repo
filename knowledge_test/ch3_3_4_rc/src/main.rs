use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn sum(&self) -> i32 {
        match self {
            Cons(value, next) => value + next.sum(),
            Nil => 0,
        }
    }
}

fn main() {
    // 任务 1：创建一个可被共享的 Rc<List>。
    let shared_tail = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _ = shared_tail.as_ref().sum();
    println!("count after creating shared_tail: {}", Rc::strong_count(&shared_tail));

    // 任务 2：通过 Rc::clone 共享所有权，而不是移动所有权。
    let _list_b = Cons(3, Rc::clone(&shared_tail));
    println!("count after creating list_b: {}", Rc::strong_count(&shared_tail));

    {
        let _list_c = Cons(4, Rc::clone(&shared_tail));
        println!("count after creating list_c: {}", Rc::strong_count(&shared_tail));
    }

    println!(
        "count after list_c goes out of scope: {}",
        Rc::strong_count(&shared_tail)
    );
}
