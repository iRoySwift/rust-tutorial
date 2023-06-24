// Go: 不用用共享内存来通信，要用通信来共享内存
// Rust 支持通过共享状态来实现并发；

// 使用Mutex来每次只允许一个线程来访问数据
// Mutex （mutual exclusion）互斥锁

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

pub fn run() {
    base_mutex();
    multhread_mutex();
}

fn base_mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn multhread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        // 多线程多重所有权
        // Rc只适合用单线程
        // let counter = Rc::clone(&counter);
        // Arc<T> 来进行原子引用计数 可以并发
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter: {:?}", counter.lock().unwrap());
}
