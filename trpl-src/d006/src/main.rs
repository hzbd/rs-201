
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}



// 结构体和枚举还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法。
impl Message {
    fn call(&self) {
        let msg = &self;
        println!("hello msg call. {:?}", msg);
    }
}


enum Option<T> {
    None,
    Some(T),
}


fn main() {

    // 枚举和模式匹配 enumerations/variants(成员)
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();


    let some_number = Some(5);
    let some_string = Some("a string");
    // let absent_number: Option<i32> = None;
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;


    // match 控制流运算符
    // let f1 = value_in_cents();
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }


    // if let 简单控制流
    // 结合 if 和 let，来处理只匹配一个模式的值而忽略其他模式的情况。
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }





}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Dime doda doda.");
            20
        },
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
