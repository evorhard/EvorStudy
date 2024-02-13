fn print_message(size: bool) {
    match size {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let number: i32 = 99;
    let is_greater_than_100: bool = number > 100;
    print_message(is_greater_than_100);
}
