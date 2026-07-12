struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    fn x(&self) -> &X {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<X, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f64, f64> {
    // this would be used should no generic override
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10.0 };
    println!("p x: {:#?} p y: {:#?}", p.x, p.y);

    let p1 = Point { x: "Hello", y: "World" };
    println!("p1 x: {:#?} p1 y: {:#?}", p1.x, p1.y);

    let p2 = p.mixup(p1);
    println!("p2 x: {:#?} p2 y: {:#?}", p2.x, p2.y);
}
