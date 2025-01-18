use std::sync::{Arc, Mutex};

use crate::engine::types::balance::{DollarBalance, StockBalance};


pub fn dollar_balances() -> Arc<Mutex<DollarBalance>> {
    static mut INSTANCE: Option<Arc<Mutex<DollarBalance>>> = None;

    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Arc::new(Mutex::new(DollarBalance {
                balance: 0,
                locked: 0,
            })));
        }
        INSTANCE.as_ref().unwrap().clone()
    }
}

pub fn stock_balances() -> Arc<Mutex<StockBalance>> {
    static mut INSTANCE: Option<Arc<Mutex<StockBalance>>> = None;

    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Arc::new(Mutex::new(StockBalance {
                yes: None,
                no: None,
            })));
        }
        INSTANCE.as_ref().unwrap().clone()
    }
}

pub fn reset_dollar_balance() {
    let dollar_balance = dollar_balances();
    let mut balance = dollar_balance.lock().unwrap();
    balance.balance = 0;
    balance.locked = 0;
}

pub fn reset_stock_balance() {
    let stock_balance = stock_balances();
    let mut balance = stock_balance.lock().unwrap();
    balance.yes = None;
    balance.no = None;
}