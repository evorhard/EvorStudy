enum Color {
    Brown,
    Red
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: f64,
    length: f64,
    height: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("length: {:?}", self.length);
        println!("height: {:?}", self.height);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {weight, color, dimensions}
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight)
    }
}

fn main() {
    let dimensions = Dimensions {
        width: 1.5,
        height: 7.0,
        length: 3.4,
    };

    let small_box = ShippingBox::new(5.0, Color::Brown, dimensions);
    small_box.print();
}
