struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// methods (get passed &self)
impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

// associated functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return  Rectangle {
            width: size,
            height: size,
        };
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("arturvilum@gmail.com"),
        username: String::from("arturvilum"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("yababab");

    let user2 = build_user(
        String::from("yabababa@gmail.com"),
        String::from("arturvilum")
    );

    let user3 = User {
        email: String::from("@gmail.com"),
        username: String::from("james"),
        ..user2 // defaults from user2
    };

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    let rect3 = Rectangle::square(30);
    println!("rect3 struct: {:#?}", rect3);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}