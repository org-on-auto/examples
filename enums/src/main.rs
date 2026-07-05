enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum struct
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
// enum implementation
impl Message {
    fn some_function() {
        println!("...");
    }
}

// struct can hold enums
struct IpAddr {
    kind: IpAddrKind,
    address: String,
    address_2: (i32, i32),
}

fn main() {
    // enum assign
    let four = IpAddrKind::V4;
    let siz = IpAddrKind::V6;

    // enum with complex datatypes
    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // equivalent to null
    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Some(5);
    let some_string = Some("a string");

    let x: i8 = 5;
    let y = Some(5);
    let sum = x + y.unwrap_or(0);

    let c1: Coin = Coin::Quarter;
    cent_value(c1);

    let some_value = Some(3);
    // the match is exhaustive
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    // if let syntax
    if let Some(3) = some_value {
        println!("Three");
    }
}

fn route(ip_kind: IpAddrKind) {
    return;
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn cent_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny in coins is 1");
            1
        },
        Coin::Nickel => {
            println!("Nickel in coins is 5");
            5
        },
        Coin::Dime => {
            println!("Dime in coins is 10");
            10
        },
        Coin::Quarter => {
            println!("Quarter in coins is 25");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}