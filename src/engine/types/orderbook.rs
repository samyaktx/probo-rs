use std::{cell::RefCell, collections::HashMap};

#[derive(Default)]
pub enum EntryType {
    Sell,
    #[default]
    Reverted,
}

pub enum OrderType {
    Yes,
    No,
}

#[derive(Default)]
pub struct IndividualEntry {
    pub entry_type: EntryType,
    pub quantity: u64,         
}

#[derive(Default)]
pub struct OrderEntry {
    pub total: u64,
    pub orders: HashMap<String, RefCell<IndividualEntry>>,          
}

#[derive(Default)]
pub struct Orderbook {
    pub yes: HashMap<u64, OrderEntry>, 
    pub no: HashMap<u64, OrderEntry>,
}

impl Orderbook {
    pub fn remove_yes(&mut self, price: u64) {
        self.yes.remove(&price);
    }

    pub fn remove_no(&mut self, price: u64) {
        self.no.remove(&price);
    }
}