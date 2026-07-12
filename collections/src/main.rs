fn main() {
    vec();
    vec_data_types();
    strings_in_rust();
    hash_map();
}

fn vec() {
    let a: [i32; _] = [1, 2, 3];  // stack
    let mut v: Vec<i32> = Vec::new(); // heap

    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3]; // heap, filled using macro - no need for type explicitly

    let third: &i32 = &v[2]; // accessing 3rd element - unsafe
    // println!("The third element is {:#?}", third);

    match v.get(2) { // accessing 3rd element - safe
        Some(third) => println!("The third element is {:#?}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{:#?}", i);
    }

    let mut x = vec![1, 2, 3, 4, 5];
    for i in &mut x {
        *i += 50;
    }

    for i in &v {
        println!("{:#?}", i);
    }
}

// creating a vector with multiple data types
// to access it -> we must always do match
// since we do not know the data type
fn vec_data_types() {
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(2),
        Cell::Float(10.12),
        Cell::Text(String::from("blue")),
    ];

    match &row[1] {
        Cell::Int(i) => println!("{:#?}", i),
        _ => println!("Not an integer!")
    }
}

fn strings_in_rust() {
    // Strings are stored as a collection of UTF-8 encoded bytes
    let s1: String = String::new();
    let s2: &str = "initial contents";
    let s3: String = s2.to_string();
    let s4: String = String::from("initial contents");

    let mut s: String = String::from("foo");
    s.push_str("bar");
    s.push('!');
    // foobar!

    let s5: String = String::from("Hello, ");
    let s6: String = String::from("World!");
    // let s7: String = s5 + &s6;
    let s7: String = format!("{}{}", s5, s6);
    // Hello, World!

    // each string is stored in bytes, scalar and grapheme
    // bytes
    let hello: String = String::from("Hello");
    for b in "Hello".bytes() {
        print!("{:#?}", b);
    }

    // scalar
    for c in hello.chars() {
        println!("{:#?}", c);
    }

    // grapheme
    // needs a crate
}

use std::collections::HashMap;
fn hash_map() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut score = HashMap::new();
    score.insert(blue, 10);
    score.insert(yellow, 50);

    let team_name = String::from("Blue");
    let point = score.get(&team_name);

    for (key, value) in &score {
        println!("{:#?}: {:#?}", key, value);
    }

    score.entry(String::from("Yellow")).or_insert(30);
    score.entry(String::from("Yellow")).or_insert(40);


    // count unique algorithm
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1 ;
    }
    println!("{:?}", map);
}