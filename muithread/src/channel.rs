// Channel包含 发送端 接收端
// 使用mpsc::channel函数来创建Channel
// 使用通信来共享内存

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    base();
    vec_channel();
    clone_channel();
}

fn base() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 发送多个值，看到接收者在等待
pub fn vec_channel() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("world"),
            String::from("china"),
            String::from("solana"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got : {}", received)
    }
}

// 通过克隆创建多个发送者
pub fn clone_channel() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: world"),
            String::from("1: china"),
            String::from("1: solana"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2: hi"),
            String::from("2: world"),
            String::from("2: china"),
            String::from("2: solana"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got : {}", received)
    }
}
