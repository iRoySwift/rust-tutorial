use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn match_open_txt() {
    let hello_file_result = File::open("hello.txt");
    let hello_file = match hello_file_result {
        Result::Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("./hello.txt") {
                Result::Ok(file) => file,
                Err(err) => panic!("Problem creating file {:?}", err),
            },
            other_err => {
                panic!("Problem opening file {:?}", other_err)
            }
        },
    };
    println!("{:?}", hello_file)
}

// unwrap_or_else 简化 match_open_txt
pub fn unwrap_open_txt() {
    let hello_file_result = File::open("hello.txt");
    let _file = hello_file_result.unwrap_or_else(|error| {
        if ErrorKind::NotFound == error.kind() {
            File::create("hello.txt").unwrap_or_else(|err| panic!("Problem create file{:?}", err))
        } else {
            panic!("Problem create file{:?}", error)
        }
    });
}

// unwrap 替代 match ，缺点不可以自定义错误信息
pub fn unwrap_expect_text() {
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
pub fn read_username_from_file() -> Result<String, io::Error> {
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
