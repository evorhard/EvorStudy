fn upper_case(string: &String) -> String {
    string.to_uppercase()
}

fn lower_case(string: &String) -> String {
    string.to_lowercase()
}

fn main() {
    let string = String::from("Fuck off, Asshole");
    println!("{:?}", upper_case(&string));
    println!("{:?}", lower_case(&string));
}
