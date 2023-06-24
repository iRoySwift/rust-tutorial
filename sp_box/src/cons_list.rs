use List::{Cons, Nil};

// 使用Box来获得确定大小的递归类型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn run() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
