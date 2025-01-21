use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::engine::types::orderbook::Orderbook;

#[derive(Default)]
pub struct OrderbookType {
    pub orderbook: HashMap<String, RefCell<Orderbook>>,
}

#[derive(Clone)]
pub struct OrderbookInstance {
    pub orderbook_instance: Arc<Mutex<OrderbookType>>,
}

impl OrderbookInstance {
    pub fn orderbook_instance() -> Self {
        static mut INSTANCE: Option<OrderbookInstance> = None;

        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(OrderbookInstance { orderbook_instance: Arc::new(Mutex::new(OrderbookType::default())) });
            }
            INSTANCE.as_ref().unwrap().clone()
        }
    }
    pub fn reset_orderbook() -> Self {
        let orderbook = Self::orderbook_instance();

        let orderbook_type = orderbook.orderbook_instance.clone();
        orderbook_type.lock().unwrap().orderbook.clear();
        orderbook
    }
}


