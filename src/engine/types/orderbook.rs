use std::collections::HashMap;

pub enum EntryType {
    Sell,
    Reverted,
}

pub enum OrderType {
    Yes,
    No,
}

pub struct IndividualEntry {
    pub entry_type: EntryType,
    pub quantity: u64,         
}

pub struct OrderEntry {
    pub total: u64,
    pub orders: HashMap<String, IndividualEntry>,          
}

pub struct Orderbook {
    pub yes: HashMap<u64, OrderEntry>, 
    pub no: HashMap<u64, OrderEntry>,
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            yes: HashMap::new(),
            no: HashMap::new(),
        }
    }
}