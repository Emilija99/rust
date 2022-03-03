use csv::Reader;
use csv::Writer;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::error::Error;
//use std::fmt::write;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    id: String,
    name: String,
    price: u64,
    quantity: u64,
}

#[derive(Debug)]
struct ShoppingCart {
    id: String,

    items: Vec<CartItem>,
}

#[derive(Debug)]
struct Shop {
    carts: Vec<ShoppingCart>,
    items: Vec<Item>,
}
#[derive(Debug)]
struct CartItem {
    item_id: String,
    quantity: u64,
}

impl Item {
    fn new(name: &str, price: u64, quantity: u64) -> Item {
        Item {
            id: nanoid!(),
            name: String::from(name),
            price,
            quantity,
        }
    }
}

impl ShoppingCart {
    fn new() -> ShoppingCart {
        ShoppingCart {
            items: vec![],
            id: nanoid!(),
        }
    }
    fn add_item(&mut self, item_id: &str, quantity: u64) {
        let item = self.items.iter_mut().find(|x| x.item_id == item_id);
        match item {
            Some(i) => {
                i.quantity += quantity;
            }
            None => self.items.push(CartItem {
                item_id: String::from(item_id),
                quantity,
            }),
        }
    }
    fn remove_item(&mut self, item_id: &str, quantity: u64) -> Option<&str> {
        let mut item = self.items.iter_mut().find(|x| x.item_id == item_id)?;
        let new_quantity = item.quantity - quantity;
        if new_quantity > 0 {
            item.quantity = new_quantity;
            Some("Item successfully removed")
        } else if new_quantity == 0 {
            let index = self
                .items
                .iter()
                .position(|x| x.item_id == item_id)
                .unwrap();
            self.items.remove(index);
            Some("Item successfully removed")
        } else {
            Some("You cant't remove more items than you already have")
        }
    }
}

impl Shop {
    fn new() -> Shop {
        Shop {
            carts: vec![],
            items: vec![],
        }
    }
    fn add_cart(&mut self) -> String {
        let cart = ShoppingCart::new();
        let id = cart.id.clone();
        self.carts.push(cart);
        id
    }

    fn save_store(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(file_name)?;

        for item in self.items.iter() {
            wtr.serialize(item)?;
        }
        wtr.flush()?;
        Ok(())
    }
    fn load_store(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut reader = Reader::from_path(file_name)?;
        for result in reader.deserialize() {
            let item: Item = result?;
            self.items.push(item);
        }
        Ok(())
    }

    fn add_item_to_cart(&mut self, cart_id: &str, product_id: &str, quantity: u64) -> String {
        let cart = self
            .carts
            .iter_mut()
            .find(|x| x.id == cart_id)
            .expect("Cart not found");

        let mut item = self
            .items
            .iter_mut()
            .find(|x| x.id == product_id)
            .expect("Item not found");

        if item.quantity > 0 {
            cart.add_item(&product_id, quantity);
            item.quantity -= 1;
            String::from("Item added to cart")
        } else {
            String::from("Item not available")
        }
    }

    fn remove_item_from_cart(
        &mut self,
        cart_id: &str,
        product_id: &str,
        quantity: u64,
    ) -> Option<&str> {
        let cart = self.carts.iter_mut().find(|x| x.id == cart_id)?;

        let mut item = self.items.iter_mut().find(|x| x.id == product_id)?;
        item.quantity += 1;
        cart.remove_item(&product_id, quantity)
    }

    fn checkout(&self, cart_id: &str) -> Option<Vec<(String, String)>> {
        if self.carts.iter().any(|x| x.id == cart_id) {
            let cart = self.carts.iter().find(|x| x.id == cart_id).unwrap();
            let mut items: Vec<(String, String)> = vec![];
            for cart_item in cart.items.iter() {
                let item = self
                    .items
                    .iter()
                    .find(|x| x.id == cart_item.item_id.to_string())
                    .unwrap();
                items.push((
                    format!("Item name: {}", item.name),
                    format!("Quantity: {}", cart_item.quantity),
                ));
            }

            Some(items)
        } else {
            None
        }
    }
    fn get_receipt(&self, cart_id: &str) -> Option<u64> {
        let cart = self.carts.iter().find(|x| x.id == cart_id)?;
        let mut sum: u64 = 0;
        for cart_item in cart.items.iter() {
            let item = self
                .items
                .iter()
                .find(|x| x.id == cart_item.item_id.to_string());
            if let Some(i) = item {
                sum += i.price * cart_item.quantity;
            }
        }
        Some(sum)
    }
    fn get_total(&self) -> Option<u64> {
        self.carts.iter().map(|x| self.get_receipt(&x.id)).sum()
    }
    fn close(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        self.carts = vec![];
        self.save_store(file_name)
    }
    fn get_item_id(&self, name: &str) -> Option<String> {
        if self.items.iter().any(|x| x.name == name) {
            let item = self.items.iter().find(|x| x.name == name).unwrap();
            Some(item.id.clone())
        } else {
            None
        }
    }
    fn show_carts_num(&self) -> usize {
        self.carts.len()
    }
    fn add_item(&mut self) -> String {
        let cart_id = enter_field("Enter your cart id:\n");
        let product_name = enter_field("Enter product name:\n");
        let quantity_str = enter_field("Enter how much of this product you would like to order:\n");
        let quantity: u64 = quantity_str.parse().unwrap();
        let product_id = self.get_item_id(&product_name).unwrap();
        self.add_item_to_cart(&cart_id, &product_id, quantity)
    }
    fn remove_item(&mut self) -> String {
        let cart_id = enter_field("Enter your cart id:\n");
        let product_name = enter_field("Enter product name:\n");
        let quantity_str =
            enter_field("Enter how much of this product you would like to remove from cart:\n");
        let quantity: u64 = quantity_str.parse().unwrap();
        let product_id = self.get_item_id(&product_name).unwrap();
        if let Some(i) = self.remove_item_from_cart(&cart_id, &product_id, quantity) {
            i.to_string()
        } else {
            String::from("item not found")
        }
    }
    fn add_new_item(&mut self) {
        let name = enter_field("Enter name of new item:\n");
        let price: u64 = enter_field("Enter price of item").parse().unwrap();
        let quantity: u64 = enter_field("Enter quantity of item").parse().unwrap();
        self.add_item_to_shop(Item::new(&name, price, quantity));
    }
    fn get_items(&self) -> &Vec<Item> {
        &self.items
    }
    fn add_item_to_shop(&mut self, item: Item) {
        self.items.push(item);
    }
}
fn write_items() -> Result<(), Box<dyn Error>> {
    let item1 = Item::new("bread", 70, 60);
    let item2 = Item::new("milk", 90, 50);
    let item3 = Item::new("flour", 120, 40);
    let item4 = Item::new("sugar", 120, 90);

    let mut wtr = Writer::from_path("items.csv")?;
    wtr.serialize(item1)?;
    wtr.serialize(item2)?;
    wtr.serialize(item3)?;
    wtr.serialize(item4)?;
    Ok(())
}

fn enter_field(txt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", txt);
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.replace("\n", "");
    buffer = buffer.replace(" ", "");
    buffer
}

fn main() {
    if let Err(e) = write_items() {
        eprintln!("{}", e);
    }

    let mut shop = Shop::new();
    if let Err(e) = shop.load_store("items.csv") {
        eprintln!("{}", e);
    }
    //println!("{:#?}", shop);

    let mut buffer = String::new();
    while buffer.to_lowercase().ne(&"k") {
        println!("\nEnter which command you would like to execute: \nA)Create new shopping cart\nB)Add item to cart\nC)Remove item from cart\nD)Checkout\nE)Close\nF)Show number of carts\nG)Show items in shop\nH)Show your receipt\nI)Total\nJ)Add item\nK)Exit\n");
        buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        buffer = buffer.replace("\n", "");
        buffer = buffer.replace(" ", "");
        match buffer.to_lowercase().as_str() {
            "a" => {
                println!("Your cart id: {}", shop.add_cart());
            }
            "b" => {
                println!("{}", shop.add_item());
            }
            "c" => {
                println!("{}", shop.remove_item());
            }
            "d" => {
                let cart_id = enter_field("Enter your cart id:\n");
                println!("Your cart: {:#?}", shop.checkout(&cart_id).unwrap());
            }
            "e" => {
                shop.close("items.csv").expect("Failed to close");
            }
            "f" => {
                println!("Total number of carts:{}\n", shop.show_carts_num())
            }
            "g" => {
                println!("Shop items:\n {:#?}", shop.get_items());
            }
            "h" => {
                let cart_id = enter_field("Enter your cart id:\n");
                let receipt = shop.get_receipt(&cart_id);
                if let Some(r) = receipt {
                    println!("Your receipt: {}", r);
                }
            }
            "i" => {
                if let Some(r) = shop.get_total() {
                    println!("Total:{}", r);
                }
            }
            "j" => {
                shop.add_new_item();
                println!("Item added to shop");
            }
            "k" => {
                println!("Exiting\n");
            }

            _ => {
                println!("Command not recognized\n");
            }
        }
    }
}
