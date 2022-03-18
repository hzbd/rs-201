
use std::collections::HashMap;


fn main() {

    // hashmap
    // 类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。
    //
    println!("Hello, world!");
    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    let vec2 = Vec::from([1, 2, 3, 4]);
    assert_eq!(vec1, vec2);
    println!("vec1:{:?}, vec2:{:?}", vec1, vec2);

    let third = &vec1[2];
    println!("item: {}", third);

    match vec1.get(10) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for c in "你好，世界".chars() { println!("{}", c); }
    // for b in "नमस्ते".bytes() { println!("{}", b);}

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    println!("{:?}\n {:?}", teams, initial_scores);

    // 这里 HashMap<_, _> 类型注解是必要的，因为可能 collect 为很多不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。但是对于键和值的类型参数来说，可以使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    println!("{:?}", field_name);
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let k1 = String::from("f100");
    map.entry(String::from("Blue")).or_insert(k1);
    // println!("{:?}", field_name);    // value borrowed here after move
    println!("{:?}", map);


    //
    // https://doc.rust-lang.org/rust-by-example/std/hash.html
    //
    let team_name = String::from("Blue0");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    match  score {
        Some(&str) => {
            println!("ok => {}", &str);
        }
        _ => {
            println!("Error");
        }
    }
}
