struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            age: 9,
            name: String::from("Timmy"),
            favorite_color: String::from("Blue")
        },
        Person {
            age: 9,
            name: String::from("Bob"),
            favorite_color: String::from("Red")
        },
        Person {
            age: 17,
            name: String::from("Shaniqua"),
            favorite_color: String::from("Television")
        },
    ];

    for person in people {
        if person.age < 10 {
            print(&person.name);
            print(&person.favorite_color)
        }
    }
}
