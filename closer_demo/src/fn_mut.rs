// FnMut 标准库定义
// 参数类型是&mut self，所以，这种类型的闭包是可变借用，会改变变量，但不会释放该变量。所以可以运行多次。
// #[lang = "fn_mut"]
// pub trait FnMut<Args>: FnOnce<Args> {
//     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
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

fn fn_mut<F>(mut f: F)
where
    F: FnMut() -> (),
{
    println!("fn_mut begins");
    f();
    f();
    println!("fn_mut ended");
}

pub fn run() {
    let mut e = E {
        a: "fn_once".to_string(),
    };
    // let f = || {
    //     println!("fn_mut closure call {:?}", e);
    //     e.a = "fn_mut".to_string();
    // };
    // 这样加个move，看看程序执行输出顺序有什么不同
    let f = move || {
        println!("fn_mut closure call {:?}", e);
        e.a = "fn_mut".to_string();
    };
    fn_mut(f);
    println!("main ended ");
}
