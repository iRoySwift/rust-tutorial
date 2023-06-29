pub mod string {
    use super::*;
    pub fn base_string() {
        let _mystring = String::new();
        let _mystring = String::from("hello");
        let _mystring = "hello".to_string();

        // 原始字符串字面量
        let mystring = r#"he"llo"#;
        println!("{}", mystring)
    }

    pub fn get_first_word() {
        // 全部切片
        let s = "Hello, world!";

        let word_index = first_word(s);
        println!("{}", word_index);

        let hello_str = String::from("Hello, world!");
        // 切片
        let hello = &hello_str[..5];
        let world = &hello_str[6..];
        let whole = &hello_str[..];
        let find_word = first_word(&hello_str);
        // mutable borrow occurs here
        // hello_str.clear();
        println!("{} | {} | {} | {}", hello, world, whole, find_word);
    }

    // 取第一个字符
    pub fn split_str() -> char {
        let hello = String::from("你好");
        let char = hello.chars();
        let mut first_char = ' ';
        for (i, item) in char.enumerate() {
            if i == 0 {
                first_char = item
            }
        }
        println!("{}", first_char);
        first_char
    }
    // 字节串
    pub fn bytes_str() {
        let _a = b'a';
        let s = b"this is byte string";
        println!("{:?}", s);

        let escaped = b"\x52\x75\x73\x74";
        println!("some escaped bytes:{:?}", escaped);

        // 原始字符串字面量
        let raw_bytestring = br#"\{u221D} is "not" \
        escaped here"#;
        println!("{:?}", raw_bytestring);
    }
    pub fn container_str() {
        let hello = String::from("hello world");
        let is_contains = hello.contains("world");
        println!("hello world包含world单词:{:?}", is_contains);
    }
    pub fn push_str() {
        let mut r = String::from("hello ");
        r.push('r');
        println!("追加字符 push() -> {}", r);

        r.push_str("ust!");
        println!("追加字符串 push_str() -> {}", r);
    }
    pub fn insert_str() {
        let mut insert_string = String::from("hello rust!");
        insert_string.insert(5, ',');
        println!("插入字符 insert() -> {}", insert_string);
        insert_string.insert_str(6, " I like");
        println!("插入字符串 insert_str() -> {}", insert_string);
    }
    pub fn add_str() {
        let hello = String::from("hello ");
        let rust = String::from("rust");
        let result = hello + &rust;
        let mut result = result + "!";
        result += "!!!";

        println!("连接字符串 + -> {} ", result);
    }
    pub fn format_str() {
        let s1 = "hello";
        let s2 = String::from("rust");
        let s = format!("{} {}", s1, s2);
        println!("{}", s);
    }
    // 该方法会替换所有匹配到的字符串。该方法是返回一个新的字符串，而不是操作原来的字符串。
    pub fn replace_str() {
        let string_replace = "I like rust. Learning rust is my favorite!";
        let new_string_replace = string_replace.replace("rust", "RUST");
        dbg!(new_string_replace);
    }
    // 该方法是返回一个新的字符串，而不是操作原来的字符串。
    pub fn replacen_str() {
        let string_replace = "I like rust. Learning rust is my favorite!";
        let new_string_replace = string_replace.replacen("rust", "RUST", 1);
        dbg!(new_string_replace);
    }
    // 该方法是直接操作原来的字符串，不会返回新的字符串
    pub fn replace_range_str() {
        let mut string_replace_range = String::from("I like rust!");
        string_replace_range.replace_range(7..8, "R");
        dbg!(string_replace_range);
    }
    //删除 pop，remove，truncate，clear
    // pop——删除并返回字符串的最后一个字符。
    pub fn pop_str() {
        let mut string_pop = String::from("rust pop 中文!");
        let p1 = string_pop.pop();
        let p2 = string_pop.pop();
        dbg!(p1);
        dbg!(p2);
        dbg!(string_pop);
    }
    // remove——删除并返回字符串中指定位置的字符  该方法是直接操作原来的字符串 存在返回值
    pub fn remove_str() {
        let mut string_remove = String::from("测试remove方法");
        println!(
            "string_remove: 占{}个字节",
            std::mem::size_of_val(string_remove.as_str())
        );
        let r1 = string_remove.remove(0);
        // 下面代码会发生错误
        // let r2 = string_remove.remove(1);
        // 直接删除第二个汉字
        let r3 = string_remove.remove(3);
        dbg!(r1);
        // dbg!(r2);
        dbg!(r3);
        dbg!(string_remove);
    }
    // truncate——删除字符串中从指定位置开始到结尾的全部字符
    pub fn truncate_str() {
        let mut string_truncate = String::from("测试truncate");
        string_truncate.truncate(3);
        dbg!(string_truncate);
    }
    pub fn clear_str() {
        let mut string_clear = String::from("string clear");
        string_clear.clear();
        dbg!(string_clear);
    }
    pub fn char_str() {
        let mystring = "ABCD".to_string();
        let mychars: Vec<char> = mystring.chars().collect(); // ['A', 'B', 'C', 'D']
        let mystring = String::from_iter(mychars.iter()); // String "ABCD"，保留mychars
        let mystring = String::from_iter(mychars); // String "ABCD"，不保留mychars
        dbg!(mystring);
    }
    pub fn u8_str() {
        // 从Vec<u8>构造
        let mystring = "ABCD".to_string();
        let mybytes = mystring.into_bytes();
        let mysting = String::from_utf8(mybytes).unwrap();
        // 从Bytes构造，其实就是构造Vec<u8>再调用from_utf8
        let mystring = "ABCD".to_string();
        let mybytes = mystring.bytes();
        let mystring = String::from_utf8(mybytes.collect()).unwrap();
        // 从&[u8]构造，其实就是构造Vec<u8>再调用from_utf8
        let mystring = "ABCD".to_string();
        let mybytes = mysting.as_bytes();
        let mystring = String::from_utf8(mybytes.to_vec()).unwrap();
        let mystring = String::from_utf8(mybytes.into()).unwrap();
        println!("{:?}", <&[u8] as Into<Vec<u8>>>::into(mybytes));
        println!("{:?}", mystring);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
