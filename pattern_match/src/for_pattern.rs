pub fn run() {
    let v = vec!["a", "b", "c"];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", index, value)
    }

    // let 模式匹配
    let b = 5;
    let (x, y, z) = (1, 2, 3);

    //  函数参数模式匹配
    fn foo(x: i32) {}
    fn print_coords(&(x, y): &(i32, i32)) {
        println!("Current coordinates:({},{})", x, y);
    }
    let point = (3, 4);
    print_coords(&point)
}
