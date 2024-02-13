fn add(number_one: i32, number_two: i32) -> i32{
    number_one + number_two
}

fn display(result: i32) {
    println!("{:?}", result)
}

fn main() {
    let result = add(8, 3);
    display(result);
}
