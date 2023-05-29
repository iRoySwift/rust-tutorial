use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("Hello, world!");
    // open_hello_txt();
    // unwrap_demo();
    let result = read_username_from_file();
}

// 打开文件
fn open_hello_txt() {
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file {:?}", e),
    //         },
    //         oe => panic!("Error opening file {:?}", oe),
    //     },
    // };

    // unwrap_or_else 新方法改良match
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file {:?}", error);
            })
        } else {
            panic!("Error opening file {:?}", error)
        }
    });
}

// unwrap 替代 match ，缺点不可以自定义错误信息
fn unwrap_demo() {
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Error opening file {:?}", error);
    //     }
    // };

    // let _f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("无法打开文件");
}

// ? 运算符 传播错误的一种快捷方式
fn read_username_from_file() -> Result<String, io::Error> {
    // 1.match
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // 2. ? 运算符
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // 3. 链式
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
