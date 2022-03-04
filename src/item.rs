use serde::{Deserialize, Serialize};
use nanoid::nanoid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
   pub id: String,
   pub name: String,
   pub price: u64,
   pub quantity: u64,
}

impl Item {
    pub fn new(name: &str, price: u64, quantity: u64) -> Item {
        Item {
            id: nanoid!(),
            name: String::from(name),
            price,
            quantity,
        }
    }
}