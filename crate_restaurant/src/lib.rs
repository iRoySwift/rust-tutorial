mod front_of_house;
//注意 use 只能创建 use 所在的特定作用域内的短路径。示例 7-12 将 eat_at_restaurant 函数移动到了一个叫 customer 的子模块，这又是一个不同于 use 语句的作用域，所以函数体不能编译。
pub use crate::front_of_house::hosting;

fn deliver_order() {
    println!("deliver_order")
}

pub fn run() {
    // 相对路径
    // front_of_house::hosting::add_to_waitlist();
    // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    // use crate
    hosting::add_to_waitlist();
    println!("run -> add_to_waitlist");
}

mod customer {
    // use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}
