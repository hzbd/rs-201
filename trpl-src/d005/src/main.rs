
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct AlwaysEqual;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// 关联函数（associated functions）
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}



fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("julia"),
        email: String::from("a@a.com"),
        sign_in_count: 10,
    };
    println!("{:?}", user1);
    user1.email = String::from("anotheremail@example.com");
    println!("{:?}", user1);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{:?}", user2);


    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    println!("{:#?}", user3);


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:#?}", black);


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "!! The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!(" rec1 width => {}", rect1.width);
    }

}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
