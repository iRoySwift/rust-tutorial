use std::thread;

pub fn run() {
    let v = vec![1, 2, 3];
    // move 将v所有权移到闭包里面
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}
