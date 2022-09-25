// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;



fn main() {
    let mut stocks = HashMap::new();
    stocks.insert("Chirs", 5);
    stocks.insert("Beds",3);
    stocks.insert("Tables", 2);
    stocks.insert("Couches", 0);
    stocks.insert("XA", 5); 
    let mut total_qty = 0;
    for (item,qty) in stocks.iter() {
        total_qty = total_qty + qty;
        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        }else{
            format!("{:?}",item)
        };
        println!(
            "item:{:?},stock={:?}",item,stock_count
        )
    }
    println!(
        "total_qty:{:?}",total_qty
    )
}
