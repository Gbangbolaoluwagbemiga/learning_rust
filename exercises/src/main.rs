use std::vec;

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> Vec<(&T, &U)> {
        return vec![(&self.x, &self.y)];
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10 };

    println!("p.x = {:?}", p.x());
    println!("p2.x = {:?}", p2.x());
}
