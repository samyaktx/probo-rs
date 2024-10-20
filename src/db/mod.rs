use std::borrow::Borrow;
use std::collections::HashMap;

mod user;
mod orderbook;
mod stock;

use std::sync::{Arc, RwLock};

use orderbook::OrderBook;
use stock::StockBalances;
use user::InrBalance;

pub type UserId = String;
pub type Users = Vec<UserId>;
// pub type OrderId = String;
pub type PriceLevel = u16;
pub type StockSymbol = String;


pub struct UserData(Option<Arc<RwLock<Option<InrBalance>>>>);

impl UserData {
    pub fn new(user_id: String) -> Self {
        let user_balance = InrBalance::new(&user_id);
        let user_data = Arc::new(RwLock::new(Some(user_balance)));
        Self(Some(user_data))
    }

    pub fn get_balance(&self, user_id: &String) -> u64 {
        match &self.0 {
            Some(user_data) => {
                let user_balance_info = user_data.read().unwrap();
                match &*user_balance_info {
                    Some(inr_balance) => inr_balance.get_inr_balance(user_id).balance,
                    None => 0, // or handle this case as appropriate for your use case
                }
            },
            None => 0, // or handle this case as appropriate for your use case
        }
    }
}



pub type STOCK_DB = Arc<RwLock<HashMap<StockSymbol, StockBalances>>>;
/*
const STOCK_BALANCES = {
	user1: {
	   "BTC_USDT_10_Oct_2024_9_30": {
		   "yes": {
			   "quantity": 1,
			   "locked": 0
		   }
	   }
	},
	user2: {
		"BTC_USDT_10_Oct_2024_9_30": {
		   "no": {
			   "quantity": 3,
			   "locked": 4
		   }
	   }
	}
}
*/

pub type ORDERBOOK_DB = Arc<RwLock<HashMap<StockSymbol, OrderBook>>>;
/* 
orderbooks : {
  stockSymbol: {
    yes: {
      price: {
        total: number;
        orders: {
            1: {
                userId : string, 
                type: 'buy'| 'sell', 
                quantity : number
            },
            2: {
                userId : string, 
                type: 'buy'| 'sell', 
                quantity : number
            },
        }  
        key: number; // last order means orders.2 for this example
      }
    }
    no: {
      
    }
  }
}
*/