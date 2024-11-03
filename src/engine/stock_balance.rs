use super::{StockSymbol, UserId};

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

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StockQuantity {
    pub quantity: u64,
    pub locked: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserStocks {
    pub yes: StockQuantity,
    pub no: StockQuantity,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StockBalance {
    user_stocks: HashMap<UserId, HashMap<StockSymbol, UserStocks>>, // userId -> (stockSymbol -> UserStocks)
}

impl StockBalance {
    pub fn instance() -> Arc<Mutex<StockBalance>> {
        static mut INSTANCE: Option<Arc<Mutex<StockBalance>>> = None;

        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Arc::new(Mutex::new(StockBalance {
                    user_stocks: HashMap::new(),
                })));
            }
            INSTANCE.as_ref().unwrap().clone()
        }
    }

    pub fn user_stock_exist(&self, user_id: &UserId) -> bool {
        self.user_stocks.contains_key(user_id)
    }

    // Method to get stock quantities for a user
    pub fn get_user_stocks(&self, user_id: &UserId) -> Option<&HashMap<StockSymbol, UserStocks>> {
            self.user_stocks.get(user_id)
    }

    // Method to get stock balance for a user
    pub fn get_user_stock_balance<'a>(&'a self, user_id: &'a UserId, stock_symbol: &'a StockSymbol) -> Result<&UserStocks, String> {
        if self.user_stock_exist(&user_id) {
            let user_stock = self.user_stocks.get(user_id).unwrap();
            let stock_balance = user_stock.get(stock_symbol).unwrap();
            
            Ok(stock_balance)    
        } else {
            return Err(format!("User {} stock balance doesn't exists", user_id));
        }        
    }

    // Method to increase stock quantity for a user
    pub fn buy_stock_quantity(
        &mut self,
        user_id: &UserId,
        stock_symbol: &StockSymbol,
        stock_type: &String,
        quantity: u64,
    ) {
        let user_stocks = self
            .user_stocks
            .entry(user_id.to_owned())
            .or_default()
            .entry(stock_symbol.to_owned())
            .or_default();
        if stock_type == "yes" {
            user_stocks.yes.quantity += quantity;
        } else if stock_type == "no" {
            user_stocks.no.quantity += quantity;
        }
    }

    // Method to decrease stock quantity for a user
    pub fn sell_stock_quantity(
        &mut self,
        user_id: &String,
        stock_symbol: &String,
        stock_type: &String,
        quantity: u64,
    ) -> Result<(), String> {
        let yes_balance = self
            .user_stocks
            .get(user_id)
            .unwrap()
            .get(stock_symbol)
            .unwrap()
            .yes
            .quantity;
        let no_balance = self
            .user_stocks
            .get(user_id)
            .unwrap()
            .get(stock_symbol)
            .unwrap()
            .no
            .quantity;

        if stock_type == "yes" && yes_balance < quantity
            || stock_type == "no" && no_balance < quantity
        {
            return Err("Insufficient balance".to_string());
        }
        let user_stocks = self
            .user_stocks
            .entry(user_id.to_owned())
            .or_default()
            .entry(stock_symbol.to_owned())
            .or_default();
        if stock_type == "yes" {
            user_stocks.yes.quantity -= quantity;
            Ok(())
        } else {
            user_stocks.no.quantity -= quantity;
            Ok(())
        }
    }

    // lock stock balance
    pub fn lock_stock(
        &mut self,
        user_id: &UserId,
        stock_symbol: &StockSymbol,
        stock_type: &String,
        quantity: u64,
    ) -> Result<(), String> {
        let user_stocks = self
            .user_stocks
            .entry(user_id.to_owned())
            .or_default()
            .entry(stock_symbol.to_owned())
            .or_default();

        if stock_type == "yes" && user_stocks.yes.quantity < quantity
            || stock_type == "no" && user_stocks.no.quantity < quantity
        {
            return Err("Insufficient balance".to_string());
        }
        if stock_type == "yes" {
            user_stocks.yes.quantity -= quantity;
            user_stocks.yes.locked += quantity;
            Ok(())
        } else {
            user_stocks.no.quantity -= quantity;
            user_stocks.no.locked += quantity;
            Ok(())
        }
    }

    // Unlock stock balance
    pub fn unlock_stock(
        &mut self,
        user_id: &String,
        stock_symbol: &String,
        stock_type: &String,
        quantity: u64,
    ) -> Result<(), String> {
        let user_stocks = self
            .user_stocks
            .entry(user_id.to_owned())
            .or_default()
            .entry(stock_symbol.to_owned())
            .or_default();
        if stock_type == "yes" && user_stocks.yes.locked < quantity
            || stock_type == "no" && user_stocks.no.locked < quantity
        {
            return Err("Insufficient balance".to_string());
        }
        if stock_type == "yes" {
            user_stocks.yes.locked -= quantity;
            user_stocks.yes.quantity += quantity;
            Ok(())
        } else {
            user_stocks.no.locked -= quantity;
            user_stocks.no.quantity += quantity;
            Ok(())
        }
    }
}

// TODO authenticate user before entry

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_stock_quantity() {
        let instance = StockBalance::instance();
        let mut mutex_guard_stock_balance = instance.lock().unwrap();
        let user = "user1".to_owned();
        let stock_symbol = "BTC_USDT_10_Oct_2024_9_30".to_owned();
        mutex_guard_stock_balance.buy_stock_quantity(&user, &stock_symbol, &"yes".to_owned(), 10);
        let stock_balance = mutex_guard_stock_balance.get_user_stock_balance(&user, &stock_symbol).unwrap();
        assert_eq!(stock_balance.yes.quantity, 10);
        assert_eq!(stock_balance.yes.locked, 0);
        assert_eq!(stock_balance.no.quantity, 0);
        assert_eq!(stock_balance.no.locked, 0);
    }

    #[test]
    fn test_sell_stock_quantity() {
        let instance = StockBalance::instance();
        let mut mutex_guard_stock_balance = instance.lock().unwrap();
        let user = "user2".to_owned();
        let stock_symbol = "BTC_USDT_10_Oct_2024_9_30".to_owned();
        mutex_guard_stock_balance.buy_stock_quantity(&user, &stock_symbol, &"yes".to_owned(), 10);
        mutex_guard_stock_balance.sell_stock_quantity(&user, &stock_symbol, &"yes".to_owned(), 5).unwrap();
        let stock_balance = mutex_guard_stock_balance.get_user_stock_balance(&user, &stock_symbol).unwrap();
        assert_eq!(stock_balance.yes.quantity, 5);
        assert_eq!(stock_balance.yes.locked, 0);
        assert_eq!(stock_balance.no.quantity, 0);
        assert_eq!(stock_balance.no.locked, 0);
    }

    #[test]
    fn test_lock_stock() {
        let instance = StockBalance::instance();
        let mut mutex_guard_stock_balance = instance.lock().unwrap();
        let user = "user3".to_owned();
        let stock_symbol = "BTC_USDT_10_Oct_2024_9_30".to_owned();
        mutex_guard_stock_balance.buy_stock_quantity(&user, &stock_symbol, &"yes".to_owned(), 10);
        mutex_guard_stock_balance.lock_stock(&user, &stock_symbol, &"yes".to_owned(), 5).unwrap();
        let stock_balance = mutex_guard_stock_balance.get_user_stock_balance(&user, &stock_symbol).unwrap();
        assert_eq!(stock_balance.yes.quantity, 5);
        assert_eq!(stock_balance.yes.locked, 5);
        assert_eq!(stock_balance.no.quantity, 0);
        assert_eq!(stock_balance.no.locked, 0);
    }

    #[test]
    fn test_unlock_stock() {
        let instance = StockBalance::instance();
        let mut mutex_guard_stock_balance = instance.lock().unwrap();
        let user = "user4".to_owned();
        let stock_symbol = "BTC_USDT_10_Oct_2024_9_30".to_owned();
        mutex_guard_stock_balance.buy_stock_quantity(&user, &stock_symbol, &"yes".to_owned(), 10);
        mutex_guard_stock_balance.lock_stock(&user, &stock_symbol, &"yes".to_owned(), 5).unwrap();
        mutex_guard_stock_balance.unlock_stock(&user, &stock_symbol, &"yes".to_owned(), 3).unwrap();
        let stock_balance = mutex_guard_stock_balance.get_user_stock_balance(&user, &stock_symbol).unwrap();
        assert_eq!(stock_balance.yes.quantity, 8);
        assert_eq!(stock_balance.yes.locked, 2);
        assert_eq!(stock_balance.no.quantity, 0);
        assert_eq!(stock_balance.no.locked, 0);
    }

    #[test]
    fn test_lock_unlock_stock() {
        let instance = StockBalance::instance();
        let mut mutex_guard_stock_balance = instance.lock().unwrap();
        let user = "user5".to_owned();
        let stock_symbol = "BTC_USDT_10_Oct_2024_9_30".to_owned();
        mutex_guard_stock_balance.buy_stock_quantity(&user, &stock_symbol, &"yes".to_owned(), 10);
        mutex_guard_stock_balance.lock_stock(&user, &stock_symbol, &"yes".to_owned(), 5).unwrap();
        mutex_guard_stock_balance.unlock_stock(&user, &stock_symbol, &"yes".to_owned(), 3).unwrap();
        let stock_balance = mutex_guard_stock_balance.get_user_stock_balance(&user, &stock_symbol).unwrap();
        assert_eq!(stock_balance.yes.quantity, 8);
        assert_eq!(stock_balance.yes.locked, 2);
        mutex_guard_stock_balance.lock_stock(&user, &stock_symbol, &"yes".to_owned(), 2).unwrap();
        let stock_balance = mutex_guard_stock_balance.get_user_stock_balance(&user, &stock_symbol).unwrap();
        assert_eq!(stock_balance.yes.quantity, 6);
        assert_eq!(stock_balance.yes.locked, 4);
    }

    #[test]
    fn test_lock_unlock_stock_insufficient_balance() {
        let instance = StockBalance::instance();
        let mut mutex_guard_stock_balance = instance.lock().unwrap();
        let result = mutex_guard_stock_balance.lock_stock(&"user6".to_owned(), &"BTC_USDT_10_Oct_2024_9_30".to_owned(), &"yes".to_owned(), 10);
        assert_eq!(result.is_err(), true);
    }
}
