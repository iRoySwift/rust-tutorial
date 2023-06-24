use std::{cell::RefCell, rc::Rc};
use List::{Cons, Nil};

// 将Rc<T>和RefCell<T>结合使用实现一个拥有多重所有权的可变数据

#[derive(Debug, Clone)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn run() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // 把Rc<T> 解引用为RefCell<T>
    *value.borrow_mut() += 10;

    println!("value={:?}", value);
    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("c={:?}", c);
}
