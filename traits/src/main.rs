use std::fmt::{ Display, Debug };

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd + Debug + Ord> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {:#?}", self.x);
            println!("Size is {:#?}", self.x.to_string().bytes());
        } else {
            println!("The largest number is y = {:#?}", self.y);
            println!("Size is {:#?}", self.x.to_string().bytes());
        }
    }
}

fn main() {
    let pair = Pair::new(10, 23);
    pair.cmp_display();

    let str_pair = Pair::new("Waa", "Wee");
    str_pair.cmp_display();
}