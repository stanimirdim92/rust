use std::fmt::Display;

trait Shape {
    fn shape(&self) -> String;
    fn size(&self, width: u32) -> u32 {
        width * 4
    }
}

struct Square {
    width: u32,
}

impl Square {
    fn size(&self) -> u32 {
        self.width * 2
    }
}
impl Shape for Square {
    fn shape(&self) -> String {
        String::from("square")
    }

    fn size(&self, width: u32) -> u32 {
        self.width * width
    }
}

#[derive(Copy, Clone)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T: Display, U: Display> Display for Point<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Display + Clone, U: Display + Clone> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn origin(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn show(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}


fn main() {
    let square = Square { width: 5 };
    println!("{}", square.shape());
    println!("{}", square.size());
    println!("Size from Shape trait: {}", Shape::size(&square, 6));


    let point = Point::origin(50, 10.5);

    println!("{}", point);
    println!("{}", point.x());
    println!("{}", point.y());
    println!("{}", point.show());
}


