use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//为mybox 实现Deref trait
impl<T> Deref for MyBox<T> {
    fn deref(&self) -> &T {
        &self.0
    }

    type Target = T;
}

fn hello(name: &str) {
    println!("hello {}", name);
}

pub fn run() {
    let x = 5;

    // let y = &x;
    // 把Box<T>当作引用
    // let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(x, *y); // *（y.deref()）
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 2. MyBox
    let m = MyBox::new(String::from("Rust"));

    // &m &MyBox<String>
    // deref &String
    // deref &str
    hello(&m); // Deref coercion 解引用隐式转换
    hello(&(*m)[..]); // 如果MyBox没有实现Deref需要这样传参
}
