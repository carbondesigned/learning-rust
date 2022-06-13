fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

// basically attaches to `Point` struct 
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    // let integer = Point{x: 1, y: 2};
    // let float = Point{x: 1.0, y: 2.0};

    let p = Point { x: 5, y: 10 };

    print!("p.x = {}", p.x());
}
