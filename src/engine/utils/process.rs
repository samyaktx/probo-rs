use std::collections::HashMap;

use crate::engine::{
    models::{
        balance::{dollar_balances, stock_balances}, 
        orderbook::OrderbookInstance,
    }, types::orderbook::{
        EntryType, OrderType
    }
};
use crate::engine::types::balance::DollarBalance;

pub fn validate_order(
    user_id: &String, 
    quantity: u64, 
    price: u64, 
    dollar_balance: &HashMap<String, DollarBalance>,
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

pub fn mint_opposite_stock(
    stock_symbol: String,
    price: u64,
    quantity: u64,
    user_id: String,
    order_type: OrderType,
) {
    let opposite_price = 10 - price;

    match order_type {
        OrderType::Yes => {
            let orderbook = OrderbookInstance::orderbook_instance();
            let mut orderbook_lock = orderbook.orderbook_instance.lock().expect("Failed to acquire lock");
            let order_entry = orderbook_lock.orderbook
                .entry(stock_symbol)
                .or_default();

            let mut order_entry = order_entry.borrow_mut();
            let order_entry = order_entry
                .no
                .entry(opposite_price)
                .or_default();

            order_entry.total += quantity;
            let order_entry_user = order_entry.orders.entry(user_id)
                .or_default();
            order_entry_user.borrow_mut().entry_type = EntryType::Reverted;
            order_entry_user.borrow_mut().quantity += quantity;
        },
        OrderType::No => {
            let orderbook = OrderbookInstance::orderbook_instance();
            let mut orderbook_lock = orderbook.orderbook_instance.lock().expect("Failed to acquire lock");

            let order_entry = orderbook_lock.orderbook
                .entry(stock_symbol)
                .or_default();

            let mut order_entry = order_entry.borrow_mut();
            let order_entry = order_entry
                .yes
                .entry(opposite_price)
                .or_default();

            order_entry.total += quantity;
            let order_entry_user = order_entry.orders.entry(user_id)
                .or_default();
            
            order_entry_user.borrow_mut().entry_type = EntryType::Sell;
            order_entry_user.borrow_mut().quantity += quantity;

        },
    }
}

pub fn buy_yes_option(
    user_id: &String,
    stock_symbol: &String,
    price: u64,
    quantity: u64,
) -> Result<(), String> {
    let dollar_balance = dollar_balances();
    let mut dollar_balance_mutex = dollar_balance.lock().expect("Failed to acquire lock");
    let dollar_balance = &dollar_balance_mutex.dollar_balance;
    
    if !validate_order(user_id, quantity, price, dollar_balance) {
        return Err("Invalid order".to_owned());
    }

    let dollar_balance = dollar_balance_mutex.dollar_balance.get_mut(user_id).unwrap();
    dollar_balance.balance -= quantity * price * 100;
    dollar_balance.locked += quantity * 100;

    let orderbook = OrderbookInstance::orderbook_instance();
    let mut orderbook_lock = orderbook.orderbook_instance.lock().expect("Failed to acquire lock");
    let orderbook_lock_exists = orderbook_lock.orderbook.contains_key(stock_symbol);

    if !orderbook_lock_exists {
        return Err("Stock not found".to_owned());
    }

    let orderbook_cell = orderbook_lock.orderbook.get_mut(stock_symbol).unwrap();
    let mut orderbook_lock = orderbook_cell.borrow_mut();
    let orderbook_lock_yes = orderbook_lock.yes.get(&price).unwrap();

    let orderbook_ref = orderbook_cell.borrow();
    let orderbook_lock_no = orderbook_ref.no.get(&(10 - price)).unwrap();

    let available_quantity = orderbook_lock_yes.total;
    let available_no_quantity = orderbook_lock_no.total;

    println!("available quantity is: {}", available_quantity);
    println!("available no quantity is: {}", available_no_quantity);

    let mut temp_quantity = quantity;

    if available_quantity > 0 {
        let orderbook = OrderbookInstance::orderbook_instance();
        let orderbook_instance = orderbook.orderbook_instance.lock().expect("Failed to acquire lock");
        let yes_orders = &orderbook_instance.orderbook
            .get(stock_symbol).unwrap().borrow();

        let yes_orders = &yes_orders
            .yes
            .get(&price).unwrap()
            .orders;

        for (user_id, user_entry) in yes_orders {
            if temp_quantity <= 0 {
                break;
            }

            let available = user_entry.borrow().quantity;
            let to_take = available.min(temp_quantity);
            
            user_entry.borrow_mut().quantity -= to_take;
            let mut orderbook_lock = orderbook_cell.borrow_mut();
            let orderbook_lock_yes = orderbook_lock.yes.get_mut(&price).unwrap();
            orderbook_lock_yes.total -= to_take;

            println!("temp quantity before: {}", temp_quantity);
            temp_quantity -= to_take;
            println!("temp quantity after: {}", temp_quantity);

            match user_entry.borrow().entry_type {
                EntryType::Sell => {
                    let stock_balances = stock_balances();
                    let mut stock_balances = stock_balances.lock().expect("Failed to acquire lock");
                    let stock_balances = stock_balances.stock_balance.get_mut(user_id).unwrap();
                    stock_balances.get_mut(stock_symbol).unwrap().yes.as_mut().unwrap().locked -= to_take;

                    let dollar_balance = dollar_balances();
                    let mut dollar_balance = dollar_balance.lock().expect("Failed to acquire lock");
                    let dollar_balance = dollar_balance.dollar_balance.get_mut(user_id).unwrap();
                    dollar_balance.balance += to_take * price * 100;
                },
                EntryType::Reverted => {
                    let stock_balances = stock_balances();
                    let mut stock_balances = stock_balances.lock().expect("Failed to acquire lock");
                    let stock_balances = stock_balances.stock_balance.get_mut(user_id).unwrap();
                    stock_balances.get_mut(stock_symbol).unwrap().no.as_mut().unwrap().quantity += to_take;

                    let dollar_balance = dollar_balances();
                    let mut dollar_balance = dollar_balance.lock().expect("Failed to acquire lock");
                    let dollar_balance = dollar_balance.dollar_balance.get_mut(user_id).unwrap();
                    dollar_balance.locked -= to_take * price * 100;
                }
            }

            if user_entry.borrow().quantity == 0 {
                orderbook_lock_yes.orders.remove(user_id);
            }
        }

        if orderbook_lock_yes.total == 0 {
            orderbook_lock.remove_yes(price);
        }
    }

    let no_price_exists = orderbook_lock.no.contains_key(&(10 - price));
    if available_no_quantity > 0 && no_price_exists {
        let orderbook_lock_no = &orderbook_lock.no.get_mut(&(10 - price)).unwrap().orders;

        for (user_id, user_entry) in orderbook_lock_no {
            if temp_quantity <= 0 {
                break;
            }

            let available = user_entry.borrow().quantity;
            let to_take = available.min(temp_quantity);

            user_entry.borrow_mut().quantity -= to_take;
            let mut orderbook_lock = orderbook_cell.borrow_mut();
            let orderbook_lock_no = orderbook_lock.no.get_mut(&(10 - price)).unwrap();
            orderbook_lock_no.total -= to_take;

            println!("temp quantity before: {}", temp_quantity);
            temp_quantity -= to_take;
            println!("temp quantity after: {}", temp_quantity);

            match user_entry.borrow().entry_type {
                EntryType::Sell => {
                    let stock_balances = stock_balances();
                    let mut stock_balances = stock_balances.lock().expect("Failed to acquire lock");
                    let stock_balances = stock_balances.stock_balance.get_mut(user_id).unwrap();
                    stock_balances.get_mut(stock_symbol).unwrap().no.as_mut().unwrap().locked -= to_take;

                    let dollar_balance = dollar_balances();
                    let mut dollar_balance = dollar_balance.lock().expect("Failed to acquire lock");
                    let dollar_balance = dollar_balance.dollar_balance.get_mut(user_id).unwrap();
                    dollar_balance.balance += to_take * (10 - price) * 100;
                },
                EntryType::Reverted => {
                    let stock_balances = stock_balances();
                    let mut stock_balances = stock_balances.lock().expect("Failed to acquire lock");
                    let stock_balances = stock_balances.stock_balance.get_mut(user_id).unwrap();
                    stock_balances.get_mut(stock_symbol).unwrap().yes.as_mut().unwrap().quantity += to_take;

                    let dollar_balance = dollar_balances();
                    let mut dollar_balance = dollar_balance.lock().expect("Failed to acquire lock");
                    let dollar_balance = dollar_balance.dollar_balance.get_mut(user_id).unwrap();
                    dollar_balance.locked -= to_take * (10 - price) * 100;
                }
            }

            if user_entry.borrow().quantity == 0 {
                orderbook_lock_no.orders.remove(user_id);
            }
        }

        if orderbook_lock.no.get(&(10 - price)).unwrap().total == 0 {
            orderbook_lock.remove_no(10 - price);
        }   
    }
    
    if temp_quantity > 0 {
        mint_opposite_stock(stock_symbol.to_owned(), price, temp_quantity, user_id.to_owned(), OrderType::Yes);
    }

    initialize_stock_balance(user_id.to_owned(), stock_symbol.to_owned());

    let stock_balances = stock_balances();
    let stock_balance_exists = stock_balances.lock().expect("Failed to acquire lock").stock_balance.contains_key(user_id);
    if stock_balance_exists {
        let mut stock_balances = stock_balances.lock().expect("Failed to acquire lock");
        let stock_balances = stock_balances.stock_balance.get_mut(user_id).unwrap();
        stock_balances.get_mut(stock_symbol).unwrap().yes.as_mut().unwrap().quantity += quantity - temp_quantity;
    }

    let dollar_balance = dollar_balances();
    let mut dollar_balance = dollar_balance.lock().expect("Failed to acquire lock");
    let dollar_balance = dollar_balance.dollar_balance.get_mut(user_id).unwrap();
    dollar_balance.locked -= (quantity - temp_quantity) * price * 100;

    Ok(())
}