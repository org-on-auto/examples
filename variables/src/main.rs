fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    // mut x can be changed
    x = 6;
    println!("The value of x is: {}", x);

    // constant cannot be mutated and always must have a type
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // immutable variables can be shadowed
    let y = 5;
    let y = 6;
    println!("The value of y is: {}", y);

    // integers can be signed or unsinged (8, 16, 32, 64, 128, arch)
    let a: i32 = 98_22; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)
    let f: u8 = 255; // valid, but zero'ed if 256

    // floating point
    let g: f64 = 3.14;
    let h: f32 = 1.0;

    // boolean
    let i: bool = true;

    // char
    let j: char = 'z';
    // strings
    let n: String = String::from("hello");

    // tuples
    let k: (i32, f64, bool) = (500, 3.14, true);

    // compund types
    let (x, y, z) = (500, 6.4, true);
    let m = (x, y, z);
    println!("The value of m is: {:?}", m);

    let m = k;
    println!("The value of m is: {:?}", m);

    // arrays - define type and size
    let l: [i32; 5] = [1, 2, 3, 4, 5];

    // vectors - dynamic size
    let m: Vec<i32> = Vec::new();

    // enums
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    println!("The color enum value is: {:?}", Color::Red);

    // references
    let x = 5;
    let y = &x;
    println!("The value of y is: {}", y);

    // operations
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    let product = 4 * 30;
    println!("The value of product is: {}", product);

    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);

    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    let t = true;
    let f: bool = !t;
    println!("The value of f is: {}", f);

    // functions
    my_function();
    println!("{}", fn_sum(5, 10));

    // control flow
    let number = 5;
    if number < 3 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
        println!("Counter: {}", counter);
    };
    println!("The result of the loop is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value of element is: {}", element);
    }

    for number in (1..4) {
        println!("The value of number is: {}", number);
    }

}

fn my_function() {
    println!("This is a function");
}

fn fn_sum(x: i32, y: i32) -> i32 {
    let result = x + y;
    result
}