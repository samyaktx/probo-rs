pub struct DollarBalance {
    pub balance: u64, 
    pub locked: u64,
}

pub struct StockOption {
    pub quantity: u64, 
    pub locked: u64,
}

#[derive(Default)] // Optional: Makes it easy to initialize with default values
pub struct StockBalance {
    pub yes: Option<StockOption>, // Optional fields are represented as `Option`
    pub no: Option<StockOption>,
}
