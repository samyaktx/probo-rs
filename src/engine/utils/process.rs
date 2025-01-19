use std::collections::HashMap;

use crate::engine::{models::{
    balance::{dollar_balances, stock_balances}, orderbook::orderbook
}, types::orderbook::{OrderEntry, OrderType, Orderbook}};
use crate::engine::types::balance::{StockBalance, DollarBalance};

pub fn validate_order(
    user_id: &String, 
    quantity: u64, 
    price: u64, 
    dollar_balance: HashMap<String, DollarBalance>,
) -> bool {
    if !dollar_balance.contains_key(user_id) {
        return false;
    }

    let dollar_balance = dollar_balance.get(user_id).unwrap();
    if dollar_balance.balance < quantity * price || price <= 0 {
        return false;
    }
    true
}

pub fn initialize_stock_balance(user_id: String, stock_symbol: String) {
    let stock_balances = stock_balances();
    let mut balances = stock_balances.lock().expect("Failed to acquire lock");
    
    let stock_balance = balances.stock_balance.entry(user_id)
        .or_default()
        .entry(stock_symbol)
        .or_default();

    stock_balance.no.as_mut().unwrap().quantity = 0;
    stock_balance.yes.as_mut().unwrap().quantity = 0;
}

// pub fn mint_opposite_stock(
//     stock_symbol: String,
//     price: u64,
//     quantity: u64,
//     user_id: String,
//     order_type: OrderType,
// ) {
//     let opposite_price = 10 - price;

//     match order_type {
//         OrderType::Yes => {
//             let orderbook = orderbook();
//             let mut orderbook_type = orderbook.lock().expect("Failed to acquire lock");
//             let orderbook_exists = orderbook_type.orderbook.contains_key(&stock_symbol);

//             let order_entry = orderbook_type.orderbook.get(&stock_symbol).unwrap();
//             let order_entry_exists = order_entry.no.contains_key(&opposite_price);
            
            
//         },
//         OrderType::No => {
//             let orderbook = orderbook();
//             let mut orderbook_type = orderbook.lock().expect("Failed to acquire lock");
//         },
//     }
// }