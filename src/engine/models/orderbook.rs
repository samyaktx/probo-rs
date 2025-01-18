use std::sync::{Arc, Mutex};

use crate::engine::types::orderbook::Orderbook;

pub fn orderbook() -> Arc<Mutex<Orderbook>> {
    static mut INSTANCE: Option<Arc<Mutex<Orderbook>>> = None;

    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Arc::new(Mutex::new(Orderbook::new())));
        }
        INSTANCE.as_ref().unwrap().clone()
    }
}

pub fn reset_orderbook() {
    let orderbook = orderbook();
    let mut orderbook = orderbook.lock().unwrap();
    orderbook.yes.clear();
    orderbook.no.clear();
}