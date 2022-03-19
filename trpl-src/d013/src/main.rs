
// *函数式语言功能：迭代器与闭包*
//
// 闭包（Closures），一个可以储存在变量里的类似函数的结构
// 迭代器（Iterators），一种处理元素序列的方式

use std::thread;
use std::time::Duration;


// 实现 Iterator trait 要求同时定义一个 Item 类型，这个 Item 类型被用作 next 方法的返回值类型。换句话说，Item 类型将是迭代器返回元素的类型。
// next 是 Iterator 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None。
pub trait Iterator {
    // 定义了 trait 的 关联类型
    // Item 类型将是迭代器返回元素的类型。
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 此处省略了方法的默认实现
}


fn main() {
    let val = simulated_expensive_calculation(20);

    println!("val => {}", val);

    // let a = SomeStruct::new();
    let a = "hello";
    let c = || println!("{}", a);
    c();


    // 第一行展示了一个函数定义，而第二行展示了一个完整标注的闭包定义。第三行闭包定义中省略了类型注解，而第四行去掉了可选的大括号，因为闭包体只有一行。这些都是有效的闭包定义，并在调用时产生相同的行为。调用闭包要求 add_one_v3 和 add_one_v4 必须更够编译因为会根据其用途推断其类型。

    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x| x + 1  ;

    // // 同一闭包函数的参数类型前后要一致！
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);


    // 即便 x 并不是 equal_to_x 的一个参数，equal_to_x 闭包也被允许使用变量 x，因为它与 equal_to_x 定义于相同的作用域。
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));


    // 闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 Fn trait：

    // * FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
    // * FnMut 获取可变的借用值所以可以改变其环境
    // * Fn 从其环境获取不可变的借用值



    // // 如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。
    // let x = vec![1, 2, 3];
    // let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);      // value borrowed here after move
    // let y = vec![1, 2, 3];
    // assert!(equal_to_x(y));


    let v1 = vec![1,2,3];
    // let items = v1.iter();
    // for val in items {
    //     println!("get val => {}", val);
    // }

    // 如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter
    // let mut v1_iter = v1.iter();
    // println!("iter next val => {:?}", v1_iter.next());
    // // assert_eq!(v1_iter.next(), Some(&1));
    // assert_eq!(v1_iter.next(), Some(&2));
    // assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), None);

    let v1_iter = v1.iter();
    let total:i32 = v1_iter.sum();
    println!("{}", total);
    // // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。
    // println!("iter next val => {:?}", v1_iter.next());

    // v1.iter().map(|x| x + 1);
    let v2:Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

}


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(1));
    intensity
}
