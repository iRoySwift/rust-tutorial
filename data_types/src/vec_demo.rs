use std::{collections::HashMap, io::stdin};

pub mod vec {
    use super::*;

    // 索引 VS get 处理访问越界
    // 索引：panic
    // get: 返回none
    pub fn vec_demo() {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);

        let third: &i32 = &v[2];
        println!("{}", third);

        match v.get(100) {
            Some(_) => println!("The third element is {},", third),
            None => println!("There is no third element"),
        }
    }

    // 所有权与借用规则
    // 可变引用
    // 不可变引用
    pub fn mut_demo() {
        let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut first = &v[0]; // immutable borrow occurs here
        println!("first {}", first);
        // v.push(6); // mutable borrow occurs here
        println!("The first element is {:?}", first);
    }

    // vec 遍历
    pub fn for_vec_demo() {
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

    pub fn enum_vec_demo() {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    // String vec[u8 ]:unicode
    //字节 bytes 标量值:scalar values 字形簇:grapheme clusters(字母)
    pub fn string_vec_demo() {
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
    pub fn vec_midian_mode() {
        let vec = vec![1, 2, 37, 8, 5, 9, 3, 3];
        let middle = midian(&vec);
        println!("{:?}中位数为:{:?}", &vec, middle);
        let mode = mode(&vec);
        println!("{:?}众数为:{:?}", &vec, mode);
    }

    // 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
    pub fn pig_latin_demo() {
        println!("first => {}", pig_latin("first"));
        println!("apple => {}", pig_latin("apple"));
    }

    // 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
    pub fn add_staff_to_dept_demo() {
        add_staff_to_dept();
    }
}

// 中位数
fn midian(vec: &Vec<i32>) -> i32 {
    let mut sorted_vec = vec.clone();
    sorted_vec.sort_unstable();
    let middle = sorted_vec.len() / 2;
    if sorted_vec.len() % 2 == 0 {
        (sorted_vec[middle - 1] + sorted_vec[middle]) / 2
    } else {
        sorted_vec[middle]
    }
}

// 众数
fn mode(vec: &Vec<i32>) -> &i32 {
    let mut map = HashMap::new();
    for item in vec {
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }
    let mode = map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(k, _)| k)
        .unwrap();
    mode
}
// pig latin
fn pig_latin(str: &str) -> String {
    let v = vec!["a", "o", "e", "i", "u"];
    let first_one = &str[0..1];
    let reset = &str[1..];
    match v.contains(&first_one) {
        true => format!("{}-{}", str, "hay"),
        false => format!("{}-{}", reset, "fay"),
    }
}

// 添加员工到部门
fn add_staff_to_dept() {
    let mut human_resource_map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        println!("请输入 Add {{员工}} to {{部门}}");
        if let Result::Ok(_len) = stdin().read_line(&mut input) {
            let words: Vec<&str> = input.trim().split_whitespace().collect();

            if words[0] != "Add" || words[2] != "to" {
                println!("输入错误！请重新输入。");
                continue;
            }
            let dept = words[3];
            let staff = words[1];
            let staff_in_dept = human_resource_map
                .entry(dept.to_string())
                .or_insert(Vec::new());
            staff_in_dept.push(staff.to_string());
            println!("{:?}", human_resource_map);
        } else {
            println!("input error")
        }
    }
}
