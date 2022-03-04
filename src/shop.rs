use csv::Writer;

use crate::{shopping_cart::ShoppingCart, Item};
use std::error::Error;
use csv::Reader;
use std::io;

#[derive(Debug)]
pub struct Shop {
    carts: Vec<ShoppingCart>,
    items: Vec<Item>,
}

impl Shop {
    pub fn new() -> Shop {
        Shop {
            carts: vec![],
            items: vec![],
        }
    }
   pub fn add_cart(&mut self) -> String {
        let cart = ShoppingCart::new();
        let id = cart.id.clone();
        self.carts.push(cart);
        id
    }

   pub fn save_store(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(file_name)?;

        for item in self.items.iter() {
            wtr.serialize(item)?;
        }
        wtr.flush()?;
        Ok(())
    }
    pub fn load_store(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut reader = Reader::from_path(file_name)?;
        for result in reader.deserialize() {
            let item: Item = result?;
            self.items.push(item);
        }
        Ok(())
    }

   pub fn add_item_to_cart(&mut self, cart_id: &str, product_id: &str, quantity: u64) -> &str {
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
            item.quantity -= quantity;
            "Item added to cart"
        } else {
            "Item not available"
        }
    }

    pub fn remove_item_from_cart(
        &mut self,
        cart_id: &str,
        product_id: &str,
        quantity: u64,
    ) -> Option<&str> {
        let cart = self.carts.iter_mut().find(|x| x.id == cart_id)?;

        let mut item = self.items.iter_mut().find(|x| x.id == product_id)?;
        item.quantity += quantity;
        cart.remove_item(&product_id, quantity)
    }

   pub  fn checkout(&mut self, cart_id: &str) -> Option<Vec<(String, String)>> {
        let cart = self.carts.iter_mut().find(|x| x.id == cart_id)?;
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
        cart.items = vec![];

        Some(items)
    }
    pub fn get_receipt(&self, cart_id: &str) -> Option<u64> {
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
    pub fn get_total(&self) -> Option<u64> {
        self.carts.iter().map(|x| self.get_receipt(&x.id)).sum()
    }
    pub fn close(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        self.carts = vec![];
        self.save_store(file_name)
    }
    pub fn get_item_id(&self, name: &str) -> Option<String> {
        let item = self.items.iter().find(|x| x.name == name)?;
        Some(item.id.clone())
    }
   pub  fn show_carts_num(&self) -> usize {
        self.carts.len()
    }
   pub fn add_item(&mut self) -> &str {
        let cart_id = enter_field("Enter your cart id:\n");
        let product_name = enter_field("Enter product name:\n");
        let quantity_str = enter_field("Enter how much of this product you would like to order:\n");
        let quantity: u64 = quantity_str.parse().unwrap();
        let product_id = self.get_item_id(&product_name);
        if let Some(i) = product_id {
            self.add_item_to_cart(&cart_id, &i, quantity)
        }
        else{
        "Item not found"
        }
    }
    pub fn remove_item(&mut self) -> &str {
        let cart_id = enter_field("Enter your cart id:\n");
        let product_name = enter_field("Enter product name:\n");
        let quantity_str =
            enter_field("Enter how much of this product you would like to remove from cart:\n");
        let quantity: u64 = quantity_str.parse().unwrap();
        let product_id = self
            .get_item_id(&product_name)
            .unwrap_or(String::from("Not found"));
        if let Some(i) = self.remove_item_from_cart(&cart_id, &product_id, quantity) {
            i
        } else {
            "item not found"
        }
    }
    pub fn add_new_item(&mut self) {
        let name = enter_field("Enter name of new item:\n");
        let price: u64 = enter_field("Enter price of item").parse().unwrap();
        let quantity: u64 = enter_field("Enter quantity of item").parse().unwrap();
        self.add_item_to_shop(Item::new(&name, price, quantity));
    }
    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }
    fn add_item_to_shop(&mut self, item: Item) {
        self.items.push(item);
    }
    pub fn add_quantity(&mut self, name: &str, quantity: u64) {
        let item = self.items.iter_mut().find(|x| x.name == name);
        if let Some(i) = item {
            i.quantity += quantity;
        }
    }
    pub fn delete_item(&mut self, name: &str) {
        let index = self.items.iter().position(|x| x.name == name);
        if let Some(i) = index {
            self.items.remove(i);
        }
    }
}

pub fn enter_field(txt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", txt);
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.replace("\n", "");
    buffer = buffer.replace(" ", "");
    buffer
}