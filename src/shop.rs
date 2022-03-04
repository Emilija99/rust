use csv::Writer;

use crate::{shopping_cart::ShoppingCart, Item};
use std::{error::Error, collections::HashMap};
use csv::Reader;
use std::io;

#[derive(Debug)]
pub struct Shop {
    carts: HashMap<String,ShoppingCart>,
    items: HashMap<String,Item>,
    total:u64
}

impl Shop {
    pub fn new() -> Shop {
        Shop {
            carts: HashMap::new(),
            items: HashMap::new(),
            total:0
           
        }
    }
   pub fn add_cart(&mut self) -> String {
        let cart = ShoppingCart::new();
        let id = cart.id.clone();
        self.carts.insert(cart.id.clone(),cart);
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
            self.items.insert(item.id.clone(), item);
        }
        Ok(())
    }

   pub fn add_item_to_cart(&mut self, cart_id: &str, product_id: &str, quantity: u64) -> Option<&str> {
        let cart = self.carts.get_mut(cart_id)?;
        let item=self.items.get_mut(product_id)?;

        if item.quantity > 0 {
            cart.add_item(&product_id, quantity);
            item.quantity -= quantity;
            Some("Item added to cart")
        } else {
            Some("Item not available")
        }
    }

    pub fn remove_item_from_cart(
        &mut self,
        cart_id: &str,
        product_id: &str,
        quantity: u64,
    ) -> Option<&str> {
        
        let cart = self.carts.get_mut(cart_id)?;
        let item=self.items.get_mut(product_id)?;
        
        item.quantity += quantity;
        cart.remove_item(&product_id, quantity)
    }

   pub  fn checkout(&mut self, cart_id: &str) -> Option<Vec<(String, String,String)>> {
        self.total+=self.get_receipt(cart_id).unwrap_or(0);
        let cart = self.carts.get_mut(cart_id)?;
        let mut items: Vec<(String, String,String)> = vec![];
        for cart_item in cart.items.iter() {
            let item=self.items.get(cart_item.0)?;
           
            items.push((
                format!("Item name: {}", item.name),
                format!("Quantity: {}", cart_item.1),
                format!("Price: {}",item.price)
            ));
        }
       
        cart.items = HashMap::new();
       self.save_store("items.csv").unwrap_or_else(|err| eprintln!("{}",err));

        Some(items)
    }
    pub fn get_receipt(&self, cart_id: &str) -> Option<u64> {
        let cart = self.carts.get(cart_id)?;
        let mut sum: u64 = 0;
        for cart_item in cart.items.iter() {
            let item = self.items.get(cart_item.0);
            if let Some(i) = item {
                sum += i.price * cart_item.1;
            }
        }
        Some(sum)
    }
    pub fn get_total_in_carts(&self) -> Option<u64> {
        self.carts.iter().map(|x| self.get_receipt(&x.0)).sum()
    }
    pub fn get_total(&self)->u64{
        self.total
    }
    pub fn close(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        self.carts = HashMap::new();
        self.save_store(file_name)
    }
    pub fn get_item_id(&self, name: &str) -> Option<String> {
        let (key,item) = self.items.iter().find(|x| x.1.name == name)?;
        Some(item.id.clone())
    }
   pub  fn show_carts_num(&self) -> usize {
        self.carts.len()
    }
   pub fn add_item(&mut self) -> Option<&str> {
        let cart_id = enter_field("Enter your cart id:\n");
        let product_name = enter_field("Enter product name:\n");
        let quantity_str = enter_field("Enter how much of this product you would like to order:\n");
        let quantity: u64 = quantity_str.parse().unwrap_or(0);
        let product_id = self.get_item_id(&product_name)?;
        
        self.add_item_to_cart(&cart_id, &product_id, quantity)
       
        
        
    }
    pub fn remove_item(&mut self) -> &str {
        let cart_id = enter_field("Enter your cart id:\n");
        let product_name = enter_field("Enter product name:\n");
        let quantity_str =
            enter_field("Enter how much of this product you would like to remove from cart:\n");
        let quantity: u64 = quantity_str.parse().unwrap_or(0);
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
        let price: u64 = enter_field("Enter price of item").parse().unwrap_or(0);
        let quantity: u64 = enter_field("Enter quantity of item").parse().unwrap_or(0);
        self.add_item_to_shop(Item::new(&name, price, quantity));
    }
    pub fn get_items(&self)->Vec<&Item>  {
        self.items.iter().map(|x| x.1).collect::<Vec<&Item>>()
    }
    fn add_item_to_shop(&mut self, item: Item) {
        self.items.insert(item.id.clone(), item);
    }
    pub fn add_quantity(&mut self, name: &str, quantity: u64) {
        let item = self.items.iter_mut().find(|x| x.1.name == name);
        if let Some(i) = item {
            i.1.quantity += quantity;
        }
    }
    pub fn delete_item(&mut self, name: &str)->Option<&str> {
        let item_id=self.get_item_id(name)?;
        self.items.remove_entry(&item_id);
        Some("Item removed")
       
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