
// *函数式语言功能：迭代器与闭包*
//
// 闭包（Closures），一个可以储存在变量里的类似函数的结构
// 迭代器（Iterators），一种处理元素序列的方式

use std::thread;
use std::time::Duration;

fn main() {
    let val = simulated_expensive_calculation(20);

    println!("val => {}", val);

    // let a = SomeStruct::new();
    let a = "hello";
    let c = || println!("{}", a);
    c();
}


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(3));
    intensity
}
