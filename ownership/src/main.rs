fn main() {
    // rules of ownership
    println!("=======================================");
    print_ownership_rules();

    // stack allocation vs. heap allocation
    println!("---------------------------------------");
    stack_and_heap();

    // taking ownership
    println!("---------------------------------------");
    let s: String = String::from("On the stack");
    takes_ownership(s);
    println!("We can't access s");

    // giving ownership
    println!("---------------------------------------");
    let s: String = gives_ownership();
    println!("We gave s an ownership: {}", s);

    // taking and giving ownership
    println!("---------------------------------------");
    let s2: String = String::from("We took and gave it back");
    let s3: String = takes_and_gives_back_ownership(s2);
    println!("Values s: {} s2: {}", s, s3);

    // rules of references
    println!("=======================================");
    print_reference_rules();

    // immutable references (borrowing)
    println!("---------------------------------------");
    let s1: String = String::from("We will use reference");
    let len: usize = get_length(&s1);
    println!("String '{}' has the length of {}", s1, len);

    // mutable references
    println!("---------------------------------------");
    let mut s1: String = String::from("We will use mut reference");
    println!("{}", change(&mut s1));

    // slices
    println!("=======================================");
    // ...
}

fn print_ownership_rules() {
    println!("---- Ownership rules ----");
    println!("1. Each value in Rust has a variable that is called its owner");
    println!("2. There can only be one owner at a time");
    println!("3. When owner goes out of scope, the value is dropped");
    println!("");
}

fn stack_and_heap() {
    /// EXAMPLE 1: declared on the stack (in binary)
    { // value s does not exist
        let s: &str = "hi owner in stack"; // s is valid in this scope (owner)
        println!("do stuff with s");
    } // out of scope - value is dropped

    /// EXAMPLE 2: declared on the heap
    { // value s does not exist
        let s = String::from("hi owner in heap"); // s is valid in this scope (owner)
        println!("do stuff with s");
    } // out of scope - value is dropped

    /// EXAMPLE 3: copy vs. move depending on the stack vs. binary
    // On the heap. Variables are dynamic, we can make a copy.
    let x: i32 = 5;
    let y = x; // Copy. We have both values accessible
    println!("Value x is {} and y is {}", x, y);

    // On the stack. Variables are not dynamic, we make a move.
    let s1: String = String::from("on the stack");
    let s2 = s1; // Move. We have only s2 accessible.
    println!("Value of s1 was moved to s2: {}", s2);

    // On the heap. Variables are dynamic, we can make a copy.
    let s3: &str = "on the heap";
    let s4 = s3;
    println!("Value of s3: {} and value of s4: {}", s3, s4);

    // On the stack. Variables are not dynamic, but we can call clone()
    let s5: String = String::from("on the stack, we will clone");
    let s6 = s5.clone();
    println!("Value of s5: {} and value of s6: {}", s5, s6);
}

fn takes_ownership(string: String) {
    println!("We took ownership of {}", string);
}

fn gives_ownership() -> String {
    let s: String = String::from("Giving ownership");
    return s;
}

fn takes_and_gives_back_ownership(string: String) -> String {
    return string;
}

fn get_length(string: &String) -> usize {
    return string.len();
}

fn change(string: &mut String) -> &String {
    string.push_str(" here");
    return string;
}

fn print_reference_rules() {
    println!("---- The Rules of References ----");
    println!("1. At any given time, you can either have one mutable or any number of immutable references");
    println!("2. References must always be valid");
}