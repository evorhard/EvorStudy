#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: String::from(name),
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}

fn main() {
    let child = Adult::new(15, "Bonita");
    let adult = Adult::new(21, "Banananajay");
    match child {
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        Err(e) => println!("{e}"),
    }
    match adult {
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        Err(e) => println!("{e}"),
    }
}
