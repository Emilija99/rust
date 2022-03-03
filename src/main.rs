use csv::Reader;
use csv::Writer;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::write;
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

    items: Vec<String>,
}

#[derive(Debug)]
struct Shop {
    carts: Vec<ShoppingCart>,
    items: Vec<Item>,
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
    fn item_to_string(&self) -> String {
        format!(
            "{},{},{},{}\n",
            self.id, self.name, self.price, self.quantity
        )
    }
}

impl ShoppingCart {
    fn new() -> ShoppingCart {
        ShoppingCart {
            items: vec![],
            id: nanoid!(),
        }
    }
    fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item))
    }
    fn remove_item(&mut self, item: &str) {
        if (self.items.iter().any(|x| x == item)) {
            let index = self.items.iter().position(|x| x == item).unwrap();
            self.items.remove(index);
        }
    }
    fn print_cart(&self) -> String {
        let mut result = String::new();
        for item in self.items.iter() {
            result += item;
            result += ", ";
        }
        result += "\n";
        result
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
    fn remove_cart(&mut self, index: usize) {
        self.carts.remove(index);
    }

    fn get_cart(&self, index: usize) -> Option<&ShoppingCart> {
        self.carts.get(index)
    }
    fn save_store(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(file_name)?;

        for item in self.items.iter() {
            wtr.serialize(item)?;
        }
        wtr.flush();
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
    fn add_item_to_cart(&mut self, cart_id: &str, product_id: &str) -> String {
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
        cart.add_item(&product_id);
        if item.quantity > 0 {
            item.quantity -= 1;
            String::from("Item added to cart\n")
        } else {
            String::from("Item not available\n")
        }
    }

    fn remove_item_from_cart(&mut self, cart_id: &str, product_id: &str) {
        if self.carts.iter().any(|x| x.id == cart_id) {
            let mut cart = self.carts.iter_mut().find(|x| x.id == cart_id).unwrap();
            if self.items.iter().any(|x| x.id == product_id) {
                let mut item = self.items.iter_mut().find(|x| x.id == product_id).unwrap();
                cart.remove_item(&product_id);
                item.quantity += 1;
            }
        }
    }
    fn checkout(&mut self, cart_id: &str) -> Option<String> {
        if self.carts.iter().any(|x| x.id == cart_id) {
            let mut cart = self.carts.iter_mut().find(|x| x.id == cart_id).unwrap();
            //cart.items=vec![];
            Some(cart.print_cart())
        } else {
            None
        }
    }
    fn close(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
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

        let product_id = self.get_item_id(&product_name).unwrap();
        self.add_item_to_cart(&cart_id, &product_id)
    }
    fn remove_item(&mut self) {
        let cart_id = enter_field("Enter your cart id:\n");
        let product_name = enter_field("Enter product name:\n");
        let product_id = self.get_item_id(&product_name).unwrap();
        self.remove_item_from_cart(&cart_id, &product_id);
    }
}
fn write_items() -> Result<(), Box<dyn Error>> {
    let item1 = Item::new("bread", 70, 60);
    let item2 = Item::new("milk", 90, 50);
    let item3 = Item::new("flour", 120, 40);
    let item4 = Item::new("bread", 120, 90);

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
    println!("{:#?}", shop);

    let mut buffer = String::new();
    while buffer.to_lowercase().ne(&"g") {
        println!("Enter which command you would like to execute: \nA)Create new shopping cart\nB)Add item to cart\nC)Remove item from cart\nD)Checkout\nE)Close\nF)Show number of carts\nG)Exit\n");
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
            "c" => shop.remove_item(),
            "d" => {
                let cart_id = enter_field("Enter your cart id:\n");
                println!("Your cart: {}", shop.checkout(&cart_id).unwrap());
            }
            "e" => {
                shop.close("items.csv").expect("Failed to close");
            }
            "f" => {
                println!("{}", shop.show_carts_num())
            }
            "g" => {
                println!("exiting\n");
            }

            _ => {
                println!("command not recognized\n");
            }
        }
    }
}
