/// Rust 可能发生内存泄漏
/// 例如使用Rc<T>和RefCell<T>就可能创造出循环引用，从而发生内存泄漏
/// - 每个项的引用数量不会变成0，值也不会被处理掉
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

// reference couting 引用计数
// Rc<T>
use List::{Cons, Nil};

#[derive(Debug, Clone)]
enum List {
    // Cons(i32, Box<List>),
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn run() {
    // a.clone() 深度拷贝 占用资源
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a.clone()));
    // let c = Cons(4, Box::new(a.clone()));

    // 使用Rc::clone()只是增加引用计数
    let a = Rc::new(Cons(
        5,
        RefCell::new(Rc::new(Cons(10, RefCell::new(Rc::new(Nil))))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(3, RefCell::new(Rc::clone(&a))));
    println!("a rc count after creating b = {}", Rc::strong_count(&a));
    println!("b inittial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 执行下面 a.tail() 会发生内存溢出
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
