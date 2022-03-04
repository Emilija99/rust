use std::io;

use crate::shop::{Shop, enter_field};



 fn create_new_shopping_cart(shop:&mut Shop){
    println!("Your cart id: {}", shop.add_cart());
}

fn add_item_to_cart(shop:&mut Shop){
    if let Some(i)=shop.add_item(){
        println!("{}", i);

    }
    else{
        println!("Item not found")
    }
  
}

fn remove_item_from_cart(shop:&mut Shop){
    println!("{}", shop.remove_item());
}

fn checkout(shop:&mut Shop){
    let cart_id = enter_field("Enter your cart id:\n");
    println!("Your cart: {:#?}", shop.checkout(&cart_id).unwrap_or(vec![]));
}

fn close(shop:&mut Shop){
    if let Err(e)=shop.close("items.csv"){
        println!("{}",e)

    }
    
}

fn total_number_of_carts(shop:&Shop){
    println!("Total number of carts:{}\n", shop.show_carts_num());
}

fn show_items_from_shop(shop:&Shop){
    println!("Shop items:\n {:#?}", shop.get_items());
}

fn show_cart_receipt(shop:&Shop){
    let cart_id = enter_field("Enter your cart id:\n");
                let receipt = shop.get_receipt(&cart_id);
                if let Some(r) = receipt {
                    println!("Your receipt: {}", r);
                }
}

fn show_total_in_carts(shop:&Shop){
    if let Some(r) = shop.get_total_in_carts() {
        println!("Total:{}", r);
    }
}


fn add_new_item_to_shop(shop:&mut Shop){
    shop.add_new_item();
    println!("Item added to shop");
}
fn add_quantity_to_item(shop:&mut Shop){
    shop.add_quantity(
        &enter_field("Enter name of item"),
        enter_field("Enter quantity of item:").parse().unwrap_or(0),
    );
}

fn delete_item_from_shop(shop:&mut Shop){
    shop.delete_item(&enter_field(
        "Enter name of product you would like to delete:",
    ));
}
fn show_total(shop:&Shop){
    println!("Total: {}",shop.get_total());
}

fn exit(){
    println!("Exiting\n");
}

pub fn run_shop(){
    let mut shop = Shop::new();
    let mut buffer = String::new();

    while buffer.to_lowercase().ne(&"n") {
        println!("\nEnter which command you would like to execute: \nA)Create new shopping cart\nB)Add item to cart\nC)Remove item from cart\nD)Checkout\nE)Close\nF)Show number of carts\nG)Show items in shop\nH)Show your receipt\nI)Show total in carts\nJ)Add item\nK)Add quantity to product\nL)Delete item\nM)Show total\nN)Exit\n");
        buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        buffer = buffer.replace("\n", "");
        buffer = buffer.replace(" ", "");

        match buffer.to_lowercase().as_str() {
            "a" => {
               create_new_shopping_cart(&mut shop)
            }
            "b" => {
                add_item_to_cart(&mut shop)
            }
            "c" => {
               remove_item_from_cart(&mut shop)
            }
            "d" => {
               checkout(&mut shop)
            }
            "e" => {
               close(&mut shop)
            }
            "f" => {
              total_number_of_carts(&shop)
            }
            "g" => {
               show_items_from_shop(&shop)
            }
            "h" => {
               show_cart_receipt(&shop)
            }
            "i" => {
                show_total_in_carts(&shop)
            }
            "j" => {
                add_new_item_to_shop(&mut shop)
            }
            "k" => {
                add_quantity_to_item(&mut shop)
            }
            "l" => {
               delete_item_from_shop(&mut shop)
            }
            "m"=>{
                show_total(&shop)
            }
            "n" => {
               exit()
            }

            _ => {
                println!("Command not recognized\n");
            }
        }
    }
}