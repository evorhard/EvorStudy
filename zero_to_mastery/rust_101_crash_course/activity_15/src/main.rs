enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, String::from("Billybob")),
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, String::from("Amybabamy"))
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) =>
                println!("Backstage Ticket Holder: {:?}, price: {:?}", holder, price),
            Ticket::Standard(price) => println!("Price: {:?}", price),
            Ticket::Vip(price, holder) =>
                println!("VIP Ticket Holder: {:?}, price: {:?}", holder, price),
        }
    }
}
