// unsafe Rust
// 解引用原始指针
// 调用unsafe函数或方法
// 访问或修改可变的静态变量
// 实现unsafe Trait

use core::slice;

pub fn run() {
    dereference_raw_point();
    run_split_unsafe();
    change_static_var(3);
    unsafe { println!("COUNTER: {:?}", COUNTER) }
}

/// 解引用原始指针
/// 原始指针
/// - 可变的 *mut T (*只是类型一部分 不是解引用符号)
/// - 不可变的 *const T。意味着指针在解引用后不能直接对其进行赋值
/// 与引用不同，原始指针：
/// - 允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针来忽略借用规则
/// - 无法保证能指向合理的内存
/// - 允许为null
/// - 不实现任何自动清理
/// 放弃保证的安全，换取更好的性能/与其它语言或硬件接口的能力
fn dereference_raw_point() {
    let mut num = 5;

    let r1 = &num as *const i32; // 将不可变引用转为不可变原始指针
    let r2 = &mut num as *mut i32; // 将可变引用转为可变原始指针

    // dereference of raw pointer is unsafe and requires unsafe function or block
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    // let address = 0x012345usize; // 内存地址不存在
    // let r = address as *const i32;
    // unsafe {
    //     println!("r: {}", *r);
    // }

    // 调用不安函数
    unsafe { dangerous() }
}
// 调用unsafe函数或方法
unsafe fn dangerous() {}

// split_at_mut切片 源代码 下面&mut slice[mid..]发生多次借用 报错 ；
// 需要使用unsafe方法
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..])

    // 使用原始指针
    // let ptr = slice as *mut [i32];
    let ptr = slice.as_mut_ptr();

    // from_raw_parts_mut 偏移量 切片
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
fn run_split_unsafe() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// 使用 extern 函数调用外部代码FFi
extern "C" {
    fn abs(input: i32) -> i32;
}
fn extern_test() {
    unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }
}

// 访问或修改一个可变静态变量
static mut COUNTER: u32 = 0;
fn change_static_var(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
