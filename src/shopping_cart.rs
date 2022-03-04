use std::collections::HashMap;

use nanoid::nanoid;

#[derive(Debug)]
pub struct CartItem {
    pub item_id: String,
    pub quantity: u64,
}

#[derive(Debug)]
pub struct ShoppingCart {
    pub id: String,

    pub items: HashMap<String, u64>,
}

impl ShoppingCart {
    pub fn new() -> ShoppingCart {
        ShoppingCart {
            items: HashMap::new(),
            id: nanoid!(),
        }
    }
    pub fn add_item(&mut self, item_id: &str, quantity: u64) {
       
        if let Some(total_quantity)=self.items.get_mut(item_id){

            *total_quantity += quantity;

        }
        else{
            self.items.insert(String::from(item_id),quantity);
        }
    }
    pub fn remove_item(&mut self, item_id: &str, quantity: u64) -> Option<&str> {
        let total_quantity = self.items.get_mut(item_id)?;
        let new_quantity = *total_quantity - quantity;
        if new_quantity > 0 {
            *total_quantity = new_quantity;
        } else {
            self.items.remove_entry(item_id);
        }
        Some("Item removed from cart")
    }
}
