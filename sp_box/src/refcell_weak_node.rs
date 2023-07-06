/// 防止循环引用
/// 把Rc<T>换成Weak<T>
/// - Rc::clone 为Rc<T>实例的strong_count加1，Rc<T>的实例只有在strong_count为0的时候才会被清理
/// - Rc<T>实例通过调用Rc::downgrade方法可以创建值的Weak Refrerence 弱引用 Weak<T> 智能指针
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// impl Node {
//     fn get_children(&self) -> Option<RefCell<Vec<Rc<Node>>>> {
//         Some(self.children)
//     }
// }

pub fn run() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf: strong = {:?} weak = {} after branch",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    // 调用upgrade() 把Weak<T>转成Rc<T>
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![(Rc::clone(&leaf))]),
        });

        //  调用downgrade 把branch的Rc转成Weak<T>
        // 把branch赋值给leaf weak 1
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "leaf: strong = {:?} weak = {} after branch",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        println!(
            "branch: strong = {:?} weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf: strong = {:?} weak = {} after branch",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
