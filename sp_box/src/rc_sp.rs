use std::rc::Rc;

// reference couting 引用计数
// Rc<T>
use crate::rc_sp::List::{Cons, Nil};

#[derive(Debug, Clone)]
enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

pub fn run() {
    // a.clone() 深度拷贝 占用资源
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a.clone()));
    // let c = Cons(4, Box::new(a.clone()));

    // 使用Rc::clone()只是增加引用计数
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
