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
        self.width * 3
    }
}


fn main() {
    let square = Square { width: 5 };


    println!("{}", square.shape());
    println!("{}", square.size());
    let size_from_shape_trait = Shape::size(&square, square.width);
    println!("Size from Shape trait: {}", size_from_shape_trait);
}


