use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::engine::types::orderbook::Orderbook;


pub struct OrderbookType {
    pub orderbook: HashMap<String, Orderbook>,
}

impl OrderbookType {
    pub fn new() -> Self {
        Self {
            orderbook: HashMap::new(),
        }
    }
}

pub fn orderbook() -> Arc<Mutex<OrderbookType>> {
    static mut INSTANCE: Option<Arc<Mutex<OrderbookType>>> = None;

    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Arc::new(Mutex::new(OrderbookType::new())));
        }
        INSTANCE.as_ref().unwrap().clone()
    }
}

pub fn reset_orderbook() {
    let orderbook = orderbook();
    let mut orderbook_type = orderbook.lock().unwrap();
    orderbook_type.orderbook.clear();
}