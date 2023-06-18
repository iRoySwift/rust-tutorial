// 'a 生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 悬垂引用
fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("abc");
    result.as_str()
}

// 字符串最大长度值
pub fn longest_test() {
    let string1 = String::from("abdc");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);
}
