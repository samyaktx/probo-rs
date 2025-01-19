use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::engine::types::balance::{DollarBalance, StockBalance};

pub struct DollarBalanceType {
    pub dollar_balance: HashMap<String, DollarBalance>,
}

impl DollarBalanceType {
    pub fn new() -> Self {
        Self {
            dollar_balance: HashMap::new(),
        }
    }
}

pub struct StockBalanceType {
    pub stock_balance: HashMap<String, HashMap<String, StockBalance>>,
}

impl StockBalanceType {
    pub fn new() -> Self {
        Self {
            stock_balance: HashMap::new(),
        }
    }
}

pub fn dollar_balances() -> Arc<Mutex<DollarBalanceType>> {
    static mut INSTANCE: Option<Arc<Mutex<DollarBalanceType>>> = None;

    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Arc::new(Mutex::new(DollarBalanceType::new())));
        }
        INSTANCE.as_ref().unwrap().clone()
    }
}

pub fn stock_balances() -> Arc<Mutex<StockBalanceType>> {
    static mut INSTANCE: Option<Arc<Mutex<StockBalanceType>>> = None;

    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Arc::new(Mutex::new(StockBalanceType::new())));
        }
        INSTANCE.as_ref().unwrap().clone()
    }
}

pub fn reset_dollar_balance() {
    let dollar_balance = dollar_balances();
    let mut balance = dollar_balance.lock().unwrap();
    balance.dollar_balance.clear();
}

pub fn reset_stock_balance() {
    let stock_balance = stock_balances();
    let mut balance = stock_balance.lock().unwrap();
    balance.stock_balance.clear();
}