use std::collections::HashMap;

fn main() {
    let mut store_items = HashMap::new();
    store_items.insert(String::from("Chairs"), 5);
    store_items.insert(String::from("Beds"), 3);
    store_items.insert(String::from("Tables"), 2);
    store_items.insert(String::from("Couches"), 0);

    let mut total_items = 0;

    for (store_item, amount) in store_items.iter() {
        total_items = total_items + amount;
        let stock_count = if amount == &0 {
            String::from("Out of stock")
        } else {
            format!("{:?}", amount)
        };
        println!("item={:?}, stock={:?}", store_item, stock_count);
    }
    println!("Total stock={:?}", total_items);
}
