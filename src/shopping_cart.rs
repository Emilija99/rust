use nanoid::nanoid;

#[derive(Debug)]
pub struct CartItem {
   pub item_id: String,
   pub quantity: u64,
}


#[derive(Debug)]
pub struct ShoppingCart {
   pub id: String,

   pub items: Vec<CartItem>,
}

impl ShoppingCart {
    pub fn new() -> ShoppingCart {
        ShoppingCart {
            items: vec![],
            id: nanoid!(),
        }
    }
    pub fn add_item(&mut self, item_id: &str, quantity: u64) {
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
    pub fn remove_item(&mut self, item_id: &str, quantity: u64) -> Option<&str> {
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