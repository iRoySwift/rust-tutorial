// Fn 标准库定义
// 参数类型是&self，所以，这种类型的闭包是不可变借用，不会改变变量，也不会释放该变量。所以可以运行多次。
// #[lang = "fn"]
// pub trait Fn<Args>: FnMut<Args> {
//     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
// }

// FnOnce 适用于能被调用一次的闭包，所有闭包都至少实现了这个 trait，因为所有闭包都能被调用。一个会将捕获的值移出闭包体的闭包只实现 FnOnce trait，这是因为它只能被调用一次。
// FnMut 适用于不会将捕获的值移出闭包体的闭包，但它可能会修改被捕获的值。这类闭包可以被调用多次。
// Fn 适用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包，当然也包括不从环境中捕获值的闭包。这类闭包可以被调用多次而不改变它们的环境，这在会多次并发调用闭包的场景中十分重要。

// 可以看出FnMut类型的闭包是可以运行多次的，且可以修改捕获变量的值。

#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroy struct E");
    }
}

fn fn_immut<F>(f: F)
where
    F: Fn() -> (),
{
    println!("fn_immut begins");
    f();
    f();
    println!("fn_immut ended");
}

pub fn run() {
    let e = E {
        a: "fn".to_string(),
    };
    // let f = || {
    //     println!("Fn closure calls: {:?}", e);
    // };
    // 这样加个move，看看程序执行输出顺序有什么不同
    let f = move || {
        println!("fn_immut closure call {:?}", e);
    };
    fn_immut(f);
    println!("main ended ");
}
