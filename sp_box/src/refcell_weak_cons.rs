use crate::List::{Cons, Nil};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), RefCell::new(Weak::new())));
    println!(
        "after creating a = {} weak = {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!("a.tail = {:?}", a.tail());
    let b = Rc::new(Cons(
        Rc::new(RefCell::new(4)),
        RefCell::new(Rc::downgrade(&a)),
    ));
    println!(
        "after creating b = {} weak={}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!("b.tail() = {:?}", b.tail());
    let c = Cons(Rc::new(RefCell::new(5)), RefCell::new(Rc::downgrade(&a)));
    println!(
        "after creating c = {} weak = {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );

    // 把Rc<T> 解引用为RefCell<T>
    *value.borrow_mut() += 10;

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
