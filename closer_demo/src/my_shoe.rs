// 13.7迭代器：使用闭包捕获环境

#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoes_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoes_size).collect()
}
