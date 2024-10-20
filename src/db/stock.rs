use std::collections::HashMap;
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

#[derive(Eq, Hash, PartialEq)]
enum StockType {
	Yes,
	No,
}

struct StockInfo {
	quantity: u64,
	locked: u64,
}

pub struct StockBalances {
	balances: HashMap<UserId, HashMap<StockSymbol, HashMap<StockType, StockInfo>>>,
}

impl StockBalances {
	pub fn new() -> Self {
		Self {
			balances: HashMap::new(),
		}
	}

	pub fn add_stock_balance(&mut self, user_id: UserId, stock_symbol: StockSymbol, stock_type: StockType, quantity: u64, locked: u64) {
		let stock_info = StockInfo {
			quantity,
			locked,
		};

		self.balances
			.entry(user_id)
			.or_insert(HashMap::new())
			.entry(stock_symbol)
			.or_insert_with(HashMap::new)
			.entry(stock_type)
			.or_insert(stock_info);
	}

}

