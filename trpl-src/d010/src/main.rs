

// Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。
// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

// trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。



pub trait Summary {
    // 默认实现
    // 有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("hello newart.")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// --------------------
// trait 作为参数
// 适用于短小的例子
pub fn notify1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify10(item1: &impl Summary, item2: &impl Summary) {}
pub fn notify20(item: &(impl Summary + Display)) {}

// Trait Bound 语法
// 注意：
// 泛型 T 被指定为 item1 和 item2 的参数限制，如此传递给参数 item1 和 item2 值的具体类型必须一致。
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
}
// 通过 + 指定多个 trait bound
pub fn notify4<T: Summary + Display>(item: &T) {}


// 通过 where 简化 trait bound
fn some_function<T: Display+Clone, U: Clone+Debug>(t: &T, u: &U) -> i32 {}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}


// 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {}


fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let lg = largest(&number_list);
    // println!("{:?}", lg);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best \
    //          hockey team in the NHL.",
    //     ),
    // };
    // println!("New article available! {}", article.summarize());

}


// 在函数定义中使用泛型
fn largest<T> (list: &[T]) -> T
    where T: PartialOrd+Copy {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


// 结构体定义中的泛型
struct Point<T> {
    x: T,
    y: T,
}
