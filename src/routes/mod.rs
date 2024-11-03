use axum::{
    routing::{delete, get, post}, 
    Router
};

mod user;
mod symbol;
mod balance;
mod orderbook;
mod reset;
mod onramp;
mod order;

use user::create_user_id;
use symbol::create_stock_symbol;
use balance::{get_user_inr_balance, 
    get_stock_balance, 
    get_inr_balance, 
    get_user_stock
};
use orderbook::get_orderbook;
use reset::reset_in_memory;
use onramp::onramp_amount;

pub fn create_routes() -> Router<()> {
    Router::new()
        .route("/", get(hello_world)) // done
        .route("/user/create/:user_id", post(create_user_id)) // done
        .route("/symbol/create/:stock_symbol", post(create_stock_symbol)) // done
        .route("/orderbook", get(get_orderbook)) // done
        .route("/balance/inr", get(get_inr_balance)) // done
        .route("/balance/stock", get(get_stock_balance)) 
        .route("/rest", delete(reset_in_memory))
        .route("/balance/inr/:user_id", get(get_user_inr_balance)) // done
        .route("/onramp/inr", post(onramp_amount))
        .route("/balance/stock/:user_id", get(get_user_stock)) // done
}


pub async fn hello_world() -> String {
    "hello, world".to_owned()
}