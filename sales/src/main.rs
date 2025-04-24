use sales::Cart;
use sales::Store;

fn main() {
    let store = Store::new(vec![
        ("Apple".to_string(), 1.0),
        ("Banana".to_string(), 2.0),
        ("Orange".to_string(), 3.0),
        ("Mango".to_string(), 4.0),
        ("Peach".to_string(), 5.0),
    ]);

    let mut cart = Cart::new();

    cart.insert_item(&store, "Apple".to_string());
    cart.insert_item(&store, "Banana".to_string());
    cart.insert_item(&store, "Orange".to_string());
    cart.insert_item(&store, "Mango".to_string());
    cart.insert_item(&store, "Peach".to_string());

    let receipt = cart.generate_receipt();

    println!("Receipt: {:?}", receipt);
}
