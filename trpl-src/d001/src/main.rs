// use std::io;



fn main() {
    println!("Hello, world!");

    let is_true = true;
    let is_false = false;
    let a = "abc";
    // let a2 = &a;
    let b = 32;
    let c = 2.0;
    let y: f32 = 3.0;

    println!("{} {} {}", is_true, is_false, a);
    println!("{}, {}, {}", b,c, y);
    println!("{} -> {:?}", a, &b);

    let difference = 95.5 - 4.3;
    println!("{}", difference);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    println!("The value of y is: {}", tup.2);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let second = a[1];
    println!("The value is: {}", second);


    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);


    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!(
    //     "The value of the element at index {} is: {}",
    //     index, element
    // );


    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);


    let x = five();
    println!("The value of x is: {}", x);


    let number = 10;
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }
    if number > 10 {
        println!(" big than 10.");
    } else if number - 2 == 8 {
        println!(" yes, it's 8.");
    } else {
        println!(" less than 10.");
    }


    let condition_state = true;
    let number = if condition_state {
        5
    } else {
        6
    };
    println!("The value of number is ---> {}", number);


    let mut counter = 0;
    // 循环标签 'counting_up
    'counting_up: loop {
        println!("hello!");
        counter += 1;
        println!("counter! -> {}", counter);
        if counter >6 && counter <=8 {
            println!("fp +++");
        } else if counter >=10 {
            break 'counting_up
        }
    }


    let mut counter1 = 90;
    let result = loop {
        counter1 += 1;
        if counter1 == 10 {
            break counter1 * 2;
        } else if counter1 >= 100 {
            println!("oh no...");
            break counter /2
        }
    };
    println!("The result is {}", result);


    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is:::: {}", element);
    }


    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!+++");

}


fn five() -> i32 { 5 }
