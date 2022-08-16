#[derive(Debug, Clone, PartialEq)]
struct User {
    username: String,
    email: String,
    signed_int_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        signed_int_count: 1,
        active: true,
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return Rectangle { width: size, height: size };
    }
}

fn main() {
    let user1 = User {
        username: "Bogan".to_string(),
        email: "boagan@gmail.com".to_string(),
        signed_int_count: 1,
        active: true,
    };

    let _name = user1.username.clone();

    println!("{:?}", user1);


    // structs are immutable by default.
    // for immutability:

    let mut user = user1.clone();

    user.username = String::from("NewUserName");
    println!("{:#?}", user);

    let user2 = build_user(
        String::from("kyle@email.com"),
        String::from("Kyle"));

    println!("{:#?}", user2);

    let user3 = User {
        email: String::from("james@mail.com"),
        username: String::from("James"),
        ..user2 // carry over all remaining values from user2
    };

    println!("{:#?}", user3);

    // tuple struct
    // anonymous field accessors generated
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle { width: 30, height: 50 };
    println!("{:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
    );

    let rect1 = Rectangle { width: 20, height: 40 };
    let rect2 = Rectangle { width: 40, height: 50 };

    println!("rect can hold rect1 {} ", rect.can_hold(&rect1));
    println!("rect can hold rect2 {} ", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(42);

    println!("{:#?}", rect3);


    println!(
        "The area of the rectangle is {} square pixels",
        rect3.area()
    );
}


