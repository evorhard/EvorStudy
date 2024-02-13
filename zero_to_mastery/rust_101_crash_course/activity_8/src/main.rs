#[derive(Debug)]
enum Flavor {
    Citrus,
    Orange,
    Mango,
}

struct Drink {
    flavor: Flavor,
    fluid_ounces: f64,
}

fn print_drink_information(drink: Drink) {
    match drink.flavor {
        Flavor::Citrus => println!("Citrus"),
        Flavor::Orange => println!("Orange"),
        Flavor::Mango => println!("Mango"),
    }

    println!("fluid_ounces: {:?}", drink.fluid_ounces);
}

fn main() {
    let citrus = Drink {
        flavor: Flavor::Citrus,
        fluid_ounces: 0.16,
    };
    print_drink_information(citrus);
}
