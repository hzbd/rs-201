//
// 测试单个用例的命令：
// cargo test --package d013 --lib -- tests::calling_next_directly
//

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// impl Config {
//     // 请记住 env::args 返回值的第一个值是程序的名称。我们希望忽略它并获取下一个值
//     pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
//         args.next();

//         let query = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a query string"),
//         };

//         let filename = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a file name"),
//         };

//         let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

//         Ok(Config {
//             query,
//             filename,
//             case_sensitive,
//         })
//     }
// }

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     contents
//         .lines()
//         .filter(|line| line.contains(query))
//         .collect()
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!("{:?}", sum);
        println!("++++++");
        // assert_eq!(18, sum);
    }
}
