/*
// Old ORDERBOOK Schema
const ORDERBOOK = {
   "BTC_USDT_10_Oct_2024_9_30": {
        "yes": {
            "9.5": {
                "total": 12,
                orders: {
                    "user1": 2,
                    "user2": 10
                }
                type: buy/sell 
            },
            "8.5": {
                "total": 12,
                "orders": {
                    "user1": 3,
                    "user2": 3,
                    "user3": 6
                }
                type: buy/sell 
            },
        },
        "no": {
        
        }
   }
}

// new ORDERBOOK schema
const ORDERBOOK = {
  "BTC_USDT_10_Oct_2024_9_30": {
    "yes": {
      "7.5": {
        "buy": {
          "total": 11,
          "orders": { 
            1: { "krishna": 4 },
            2: { "samyakt": 7 }
          },
          "key": 2
        },
        "sell": {
          "total": 7,
          "orders": { 
            1: { "krishna": 7 },
          },
          "key": 1
        }
      }
    },
    "no": {
      "4.5": {
        "buy": {
          "total": 17;
          "orders": { 
            1: { "userId": 10 },
            2: { "krishna": 7 }
          }
          "key": 2
        },
        "sell": {
          "total": 24;
          "orders": { 
            1: { "userId": 8 },
            2: { "krishna": 4 },
            3: { "samyakt", 12 }
          }
          "key": 3
        }
      }
    }


    
    no: {
      price: {
        total: number;
        orders: {
            1: { userId, stock_quote: 'buy'| 'sell',  quantity }
            2: { userId, quantity }
        }  
        key: 2;
      }
    }
  }
}

  
*/

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::engine::{StockSymbol, Price};

#[derive(Debug)]
enum OrderType {
    Buy,
    Sell,
}

pub enum StockType {
    Yes,
    No,
}

#[derive(Debug)]
struct Order {
    user_id: String,
    order_type: OrderType,
    quantity: u64,
}

#[derive(Debug)]
struct PriceEntry {
    total: u64,
    orders: HashMap<u64, Order>,
    key: u64,
}

#[derive(Debug)]
pub struct StockOrders {
    yes: HashMap<Price, PriceEntry>,
    no: HashMap<Price, PriceEntry>,
}

#[derive(Debug)]
pub struct OrderBook {
    pub orders: HashMap<StockSymbol, StockOrders>,
}

impl OrderBook {
    pub fn instance() -> Arc<Mutex<OrderBook>> {
        static mut INSTANCE: Option<Arc<Mutex<OrderBook>>> = None;
        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Arc::new(Mutex::new(OrderBook {
                    orders: HashMap::new(),
                })));
            }
            INSTANCE.as_ref().unwrap().clone()
        }
    }

    // create market
    pub fn create_market(&mut self, stock_symbol: StockSymbol) -> Result<(), String> {
        if self.orders.contains_key(&stock_symbol) {
            return Err(format!("Market for {} already exists.", stock_symbol));
        }
        self.orders.entry(stock_symbol).or_insert(StockOrders {
            yes: HashMap::new(),
            no: HashMap::new(),
        });
        Ok(())
    }

    pub fn get_orderbook(&self) -> &HashMap<StockSymbol, StockOrders> {
        &self.orders
    }


    fn process_order(
        &mut self,
        user_id: String,
        stock_symbol: String,
        stock_type: StockType,
        price: u64,
        order_type: OrderType,
        quantity: u64,
    ) -> Result<(), String> {
        // Ensure the market exists
        if !self.orders.contains_key(&stock_symbol) {
            return Err(format!("Market for {} does not exist.", stock_symbol));
        }

        let stock_orders = self.orders.get_mut(&stock_symbol).unwrap();

        // Determine the price entry based on stock type
        let price_entry = match stock_type {
            StockType::Yes => stock_orders.yes.entry(price).or_insert_with(|| PriceEntry {
                total: 0,
                orders: HashMap::new(),
                key: 0,
            }),
            StockType::No => stock_orders.no.entry(price).or_insert_with(|| PriceEntry {
                total: 0,
                orders: HashMap::new(),
                key: 0,
            }),
        };

        // Update the price entry and add the order
        price_entry.total += quantity;
        price_entry.orders.insert(
            price_entry.key,
            Order {
                user_id,
                order_type,
                quantity,
            },
        );
        price_entry.key += 1;

        Ok(())
    }

    // order buy yes
    pub fn order_buy_yes(
        &mut self,
        user_id: String,
        stock_symbol: String,
        price: Price,
        quantity: u64,
    ) {
        self.process_order(
            user_id,
            stock_symbol,
            StockType::Yes,
            price,
            OrderType::Buy,
            quantity,
        ).ok();
    }

    // Order buy no
    pub fn order_buy_no(
        &mut self,
        user_id: String,
        stock_symbol: String,
        price: Price,
        quantity: u64,
    ) {
        self.process_order(
            user_id,
            stock_symbol,
            StockType::No,
            price,
            OrderType::Buy,
            quantity,
        ).ok();
    }

    // Order sell yes
    pub fn order_sell_yes(
        &mut self,
        user_id: String,
        stock_symbol: String,
        price: Price,
        quantity: u64,
    ) {
        self.process_order(
            user_id,
            stock_symbol,
            StockType::Yes,
            price,
            OrderType::Sell,
            quantity,
        ).ok();
    }

    // Order sell no
    pub fn order_sell_no(
        &mut self,
        user_id: String,
        stock_symbol: String,
        price: Price,
        quantity: u64,
    ) {
        self.process_order(
            user_id,
            stock_symbol,
            StockType::No,
            price,
            OrderType::Sell,
            quantity,
        ).ok();
    }

    // cancel order
    // pub fn cancel_order(&mut self, user_id: UserId, stock_symbol: StockSymbol, price: Price, quantity: u64) {
    //     // TODO: Implement cancel order

    // }

    // instant buy

    // instant sell

    // getBestprice

    // match order
}


// #[derive(Clone, PartialEq, Eq)]
// pub enum OrderType {
//     Buy,
//     Sell
// }

// #[derive(Clone, PartialEq, Eq)]
// pub struct Order {
//     user_id: String,
//     types: OrderType,
//     quantity: u64,
// }

// #[derive(Clone, PartialEq, Eq)]
// pub struct ListOrders {
//     total: u64,
//     orders: HashMap<u32, Order>, 
//     key: u32
// }

// // Struct to represent bids or asks for "yes" or "no"
// #[derive(Clone, PartialEq, Eq)]
// struct StockOrders {
//     price_level: HashMap<PriceLevel, ListOrders>,
// }

// // Struct representing the order book for a specific market and time
// #[derive(Clone, PartialEq, Eq)]
// struct OrderBookEntry {
//     yes: StockOrders,
//     no: StockOrders,
// }

// // Main order book structure
// #[derive(Clone, PartialEq, Eq)]
// pub struct OrderBook {
//     order_book: HashMap<StockSymbol, OrderBookEntry>,
// }

// impl ListOrders {
//     pub fn new() -> Self {
//         Self {
//             total: 0,
//             orders: HashMap::new(),
//             key: 0
//         }
//     }

//     pub fn add_list_order(&mut self, key: u32, user_id: UserId, order_type: OrderType, quantity: u64) {
//         self.total += quantity;

//         let order = Order { user_id: user_id.clone(), types: order_type, quantity };

//         let _ = self.orders.entry(key).or_insert(order).clone();
//     }

//     pub fn remove_list_order(&mut self, key: u32, user_id: UserId, order_type: OrderType, quantity: u64) {
//         self.total -= quantity;

//         let order = Order { user_id: user_id.clone(), types: order_type, quantity };
//         if let Some(existing_order) = self.orders.get_mut(&key) {
//             existing_order.quantity -= quantity;
//         }

//         let _ = self.orders.entry(key).or_insert(order).clone();
//     }

//     pub fn get_list_orders(&self) -> &HashMap<u32, Order> {
//         &self.orders
//     }
// }



// impl OrderBook {
//     pub fn new() -> Self {
//         Self {
//             order_book: HashMap::new(),
//         }
//     }

//     pub fn add_order(&mut self, stock_symbol: StockSymbol, price_level: PriceLevel, user_id: UserId, quantity: u64) -> OrderBook {
//         let entry = self.order_book.entry(stock_symbol.clone()).or_insert(OrderBookEntry { 
//             yes: StockOrders { price_level: HashMap::new() }, 
//             no: StockOrders { price_level: HashMap::new() } 
//         });
//         let stock_orders = entry.yes.price_level.entry(price_level.clone()).or_insert_with(|| ListOrders::new());
//         stock_orders.total += quantity;
//         stock_orders.add_list_order(stock_orders.key, user_id, OrderType::Buy, quantity);
//         stock_orders.key += 1;
//         self.clone()
//     }

// // new ORDERBOOK schema
// // orderbooks : {
// //   stockId: {
// //     yes: {
// //       price: {
// //         total: number;
// //         orders: { 1: { userId, type: 'buy'| 'sell', quantity } }  
// //         key: number;
// //       }
// //     }
// //     no: {
      
// //     }
// //   }
// // }

//     pub fn remove_order(&mut self, stock_symbol: StockSymbol, price_level: PriceLevel, user_id: UserId, quantity: u64) {
//         if !self.order_book.contains_key(&stock_symbol) {   
//             return;
//         }
//         let entry = self.order_book.get_mut(&stock_symbol).unwrap();
//         if !entry.yes.price_level.contains_key(&price_level) {
//             return;
//         }
//         let stock_orders = entry.yes.price_level.get_mut(&price_level).unwrap();
//         if !stock_orders.orders.contains_key(&stock_orders.key) {  
//             return;
//         }
//         let current_quantity = stock_orders.orders.get(&stock_orders.key).unwrap();

//         if current_quantity.quantity < quantity {
//             return;
//         }
//         stock_orders.total -= quantity;
//         stock_orders.orders.get_mut(&stock_orders.key).unwrap().quantity -= quantity;  
//         if stock_orders.total == 0 {
//             stock_orders.orders.remove(&stock_orders.key);
//         }
//     }

//     pub fn get_orders(&self, stock_symbol: StockSymbol, price_level: PriceLevel) -> Option<&ListOrders> {
//         if !self.order_book.contains_key(&stock_symbol) {   
//             return None;
//         }
//         let entry = self.order_book.get(&stock_symbol).unwrap();
//         if !entry.yes.price_level.contains_key(&price_level) {
//             return None;
//         }
//         let orders = entry.yes.price_level.get(&price_level).unwrap();
//         Some(orders)    
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_list_orders() {
//         let mut list_orders = ListOrders::new();
//         list_orders.add_list_order(1, "user1".to_string(), OrderType::Buy, 10);
//         list_orders.add_list_order(2, "user2".to_string(), OrderType::Buy, 20);
//         list_orders.add_list_order(3, "user3".to_string(), OrderType::Buy, 30);

//         assert_eq!(list_orders.total, 60);
//         assert_eq!(list_orders.orders.get(&1).unwrap().quantity, 10);
//         assert_eq!(list_orders.orders.get(&2).unwrap().quantity, 20);
//         assert_eq!(list_orders.orders.get(&3).unwrap().quantity, 30);
//     }

//     #[test]
//     fn test_remove_list_orders() {
//         let mut list_orders = ListOrders::new();
//         list_orders.add_list_order(1, "user1".to_string(), OrderType::Buy, 10);
//         list_orders.add_list_order(2, "user2".to_string(), OrderType::Buy, 20);

//         list_orders.remove_list_order(1, "user1".to_string(), OrderType::Buy, 5);
//         assert_eq!(list_orders.total, 25);
//         assert_eq!(list_orders.orders.get(&1).unwrap().quantity, 5);
//     }

//     #[test]
//     fn test_get_list_orders() {
//         let mut list_orders = ListOrders::new();
//         list_orders.add_list_order(1, "user1".to_string(), OrderType::Buy, 10);
//         list_orders.add_list_order(2, "user2".to_string(), OrderType::Buy, 20);
//         let orders = list_orders.get_list_orders();
//         assert_eq!(orders.len(), 2);
//         assert_eq!(orders.get(&1).unwrap().quantity, 10);
//         assert_eq!(orders.get(&2).unwrap().quantity, 20);
//     }
// }
