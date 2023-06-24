// 用到模式的地方
// 条件if let 表达式
// 替代只有一个匹配项的match
// if let 可以使用 else if

pub fn run() {
    let favorite_color: Option<&str> = None;
    let is_tuesday: bool = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background {}", age)
        } else {
            println!("Using orange as the background {}", age)
        }
    } else {
        println!("Using blue as the background")
    }
}
