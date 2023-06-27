use std::collections::HashMap;

fn main() {
    string_vec_demo();
    // for_vec_demo();
    // count_word_demo();
}
// 索引 VS get 处理访问越界
// 索引：panic
// get: 返回none
fn vec_demo() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let third: &i32 = &v[2];
    println!("{}", third);

    match v.get(100) {
        Some(v) => println!("The third element is {},", third),
        None => println!("There is no third element"),
    }
}

// 所有权与借用规则
// 可变引用
// 不可变引用
fn mut_demo() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut first = &v[0]; // immutable borrow occurs here
    println!("first {}", first);
    // v.push(6); // mutable borrow occurs here
    println!("The first element is {:?}", first);
}

// vec 遍历
fn for_vec_demo() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 解引用
    }
    for i in v {
        println!("{}", i);
    }
}

// enum vec可以存放不同枚举类型
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_vec_demo() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

// String vec[u8 ]:unicode
//字节 bytes 标量值:scalar values 字形簇:grapheme clusters(字母)
fn string_vec_demo() {
    let w = "Hello world";
    for b in w.bytes() {
        println!("{}", b)
    }
    // unicode
    for c in w.chars() {
        println!("{}", c)
    }
}

// HashMap<K,V>
fn count_word_demo() {
    let text = "hello world wonderful world";
    let mut map: HashMap<&str, _> = HashMap::new();
    for word in text.split_whitespace() {
        let mapEntry = map.entry(word);
        println!("{:#?}", mapEntry);
        let count = mapEntry.or_insert(0);
        println!("{:?}", count);
        *count += 1;
    }
    println!("{:#?}", map)
}
