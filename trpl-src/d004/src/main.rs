fn main() {

    // 所有权
    let s1 = "hello";
    {
        let s2 = "hello2";
        println!("{}", s2)
    }
    println!("{}", s1);


    let mut s = String::from("hello");
    s.push_str("00000sdfdasf");
    println!("{}", s);



    // 移动
    // 克隆
    // 拷贝(copy trait) - 只在栈上的数据
    // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

    // # *所有权原则*：
    // * Rust 中每一个值都 有且只有 一个所有者(变量)
    // * 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)


    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    let s1 = String::from("hello");
    let s2 = s1;


    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s1 = {}, s2 = {}", s3, s4);



    // 引用与借用(referencing & borrowing)
    // 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。与指针不同，引用确保指向某个特定类型的有效值。
    //
    // 注意：与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*。我们将会在第八章遇到一些解引用运算符，并在第十五章详细讨论解引用。

    // 可变引用有一个很大的限制：在同一时间只能有一个对某一特定数据的可变引用


    // 引用的规则:
    // * 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    // * 引用必须总是有效的。

    let s5 = String::from("hello");
    let len = calculate_length(&s5);
    println!("The length of '{}' is {}.", s5, len);



    // 不能在拥有不可变引用的同时拥有可变引用
    // 有不可变引用存在时，不能同时存在可变引用！！！
    let mut s6 = String::from("hello");

    let r1 = &s6; // 没问题,只读
    let r2 = &s6; // 没问题,只读
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    // 非词法作用域生命周期 NLL
    let r3 = &mut s6; // 没问题
    println!("{}", r3);


    // 悬垂引用（Dangling References）
    let reference_to_nothing = no_dangle();
    println!("danging --> {}", reference_to_nothing);


    let x = 5;
    let y = &x;

    let s100 = "hello fj";

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x:{}, *y:{}", x, *y);
    println!("{}", s100);


    let mut s = String::from("hello world");
    let (word, len) = first_word(&s);
    println!("    word => {:?}", word);
    println!("word len => {}", len);
    s.clear();
    // let len = s.len();
    // let slice = &s[3..len];
    // let slice = &s[..];
    // println!("slice: {}", slice);


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}


fn calculate_length(s: &str) -> usize {
    s.len()
}

fn no_dangle() -> String {
    let s20 = String::from("hello");
    // &s  ！！！
    s20
}

// fn first_word(s: &str) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i
//         }
//     }
//     s.len()
// }

fn first_word(s: &str) -> (&str, usize) {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("++++> {}", i);
            return (&s[0..i], i)
        }
    }
    // !!!if not with space return all!!!
    // println!("++++> {}", &s[..]);
    (&s[..], s.len())
}
