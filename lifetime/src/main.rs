use std::fmt::{Display, Debug};

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str where
    T: Display + Debug
{
    println!("Announcement! {:#?}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1: String = String::from("This");
    let s2: String = String::from("Not That");
    let an: &str = "boooyaka";

    let result = longest_with_an_announcement(&s1, &s2, &an);
    println!("Result is: {:#?}", result);
}