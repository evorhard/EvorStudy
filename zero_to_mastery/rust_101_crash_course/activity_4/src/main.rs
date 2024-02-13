fn main() {
    let boolbool: bool = false;

    match boolbool {
        true => println!("it's true"),
        false => println!("it's false"),
    }

    let number: i16 = 6;

    match number {
        1 => println!("The number is 1"),
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        _ => println!("The number is neither 1, 2 or 3")
    }
}
