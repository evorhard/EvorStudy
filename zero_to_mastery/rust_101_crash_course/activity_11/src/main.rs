struct GroceryItem {
    quantity: i32,
    id_number: i32,
}

fn display_quantity(grocery_item: &GroceryItem) {
    println!("{:?}", grocery_item.quantity);
}

fn display_id(grocery_item: &GroceryItem) {
    println!("{:?}", grocery_item.id_number);
}

fn main() {
    let lettuce = GroceryItem {
        quantity: 302,
        id_number: 1
    };

    display_quantity(&lettuce);
    display_id(&lettuce);
}
