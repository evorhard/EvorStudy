fn coordinate() -> (i32, i32) {
    (1, 7)
}

fn main() {
    let (x_coordinate, y_coordinate) = coordinate();

    if y_coordinate > 5 {
        println!(">5")
    } else if y_coordinate < 5 {
        println!("<5")
    } else {
        println!("=5")
    }
}
