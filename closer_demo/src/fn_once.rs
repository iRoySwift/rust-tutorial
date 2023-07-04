// FnOnce 标准库定义
// 参数类型是self，所以，这种类型的闭包会获取变量的所有权，生命周期只能是当前作用域，之后就会被释放了。
// 标准库中，Fn, FnMut, FnOnce 的实现。可以看到 Fn 继承自 FnMut, FnMut 继承自 FnOnce
// #[lang = "fn_once"]
// pub trait FnOnce<Args> {
//     type Output;
//     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
// }

// FnOnce 适用于能被调用一次的闭包，所有闭包都至少实现了这个 trait，因为所有闭包都能被调用。一个会将捕获的值移出闭包体的闭包只实现 FnOnce trait，这是因为它只能被调用一次。
// FnMut 适用于不会将捕获的值移出闭包体的闭包，但它可能会修改被捕获的值。这类闭包可以被调用多次。
// Fn 适用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包，当然也包括不从环境中捕获值的闭包。这类闭包可以被调用多次而不改变它们的环境，这在会多次并发调用闭包的场景中十分重要。

// 参数类型是self，所以，这种类型的闭包会获取变量的所有权，生命周期只能是当前作用域，之后就会被释放了。

#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroy struct E");
    }
}

fn fn_once<F>(f: F)
where
    F: FnOnce() -> E,
{
    println!("fn_once begins");
    f();
    // f();
    println!("fn_once ended");
}

pub fn run() {
    let e = E {
        a: "fn_once".to_string(),
    };
    // let f = || println!("fn_once closure call {:?}", e);
    // 这样加个move，看看程序执行输出顺序有什么不同
    let f = move || e;
    fn_once(f);
    println!("main ended");
}
