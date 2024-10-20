# Overview

The goal of this assignment is to create a basic options trading app in Node.js. The app will simulate a trading system for options by managing balances and an orderbook using in-memory variables. This assignment will help you understand state management, working with in-memory data structures, and handling basic API requests.

Example website - https://probo.in/


### Requirements:

1. **Global In-memory Variables:**
You will maintain three global variables in memory that will simulate the state of the app:

- `INR_BALANCES`: This will hold the available balance of INR for different users.

```json
const INR_BALANCES = {
  "user1": {
     balance: 10,
     locked: 0
  },
  "user2": {
     balance: 20,
     locked: 10
  }
};
```

- `ORDERBOOK`: This will store the active buy and sell orders placed by users for options trading.
```json
const ORDERBOOK = {
   "BTC_USDT_10_Oct_2024_9_30": {
			"yes": {
				"9.5": {
					"total": 12,
					orders: {
						"user1": 2,
						"user2": 10
					}
				},
				"8.5": {
					"total": 12,
					"orders": {
						"user1": 3,
						"user2": 3,
						"user3": 6
					}
				},
			},
			"no": {
			
			}
   }
}
```

- `STOCK_BALANCES`: This will track the quantity of stock held by users that they can trade options on.
```json
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
```

# Endpoints

- [ ] **Create a user**
    - Endpoint : `/user/create/:userId`
    - Method : `GET`
    - Description: Create a new user entry in INR_BALANCES with unique user Id and default 0 balances
    
- [ ] **Create a Symbol**
    - Endpoint : `/symbol/create/:stockSymbol`
    - Method : `GET`
    - Description: Create a new symbol in ORDER_BOOK with default yes and no entries

- [ ] **Get INR Balance**
    - Endpoint: `/balance/inr/:userId`
    - Method: `GET`
    - Description: Returns the INR balance of a given user.
  
- [ ] **Onramp INR**
    - Endpoint: `/onramp/inr`
    - Method: `POST`
    - Body: 
    ```json
    {
        "userId": "user1",
        "amount": 10000 // make sure amount is in paise and not Rs
    }
    ```
    - Description: Lets the user onramp INR on the platform

- [ ] **Get Stock Balance**
    - Endpoint: `/balance/stock/:userId`
    - Method: `GET`
    - Description: Returns the stock balance for a user.
  
- [ ]  **Buy the `yes` stock**
    - Endpoint: `/order/yes`
    - Method: `POST`
    - Description: Allows a user to place a buy order for options on a stock. The order will be added to the `ORDERBOOK`.
    - Input (example):
    ```json
    {
        "userId": "123",
        "stockSymbol: "BTC_USDT_10_Oct_2024_9_30",
        "quantity": 100,
        "price": 1000
    }
    ```

- [ ] **Place Sell Order**
    - Endpoint: `/order/no`
    - Method: `POST`
    - Description: Allows a user to place a sell order for options. This will also be added to the `ORDERBOOK`.
    - Input (example):
    ```json
    {
        "userId": "123",
        "stockSymbol": "ABC",
        "quantity": 100,
        "price": 1100
    }
    ```

- [ ] **View Orderbook**
    - Endpoint: `/orderbook/:stockSymbol`
    - Method: `GET`
    - Description: Returns the current buy and sell orders for a given stock.
  
- [ ] **Mint fresh tokens**  (This endpoint NOT Required)
    - Endpont: `/trade/mint/:stockSymbol`
    - Method: `POST`
    - Input
    ```json
    {
        "userId": "123",
        "stockSymbol": "ABC",
        "quantity": 100,
    }
    ```
    
- [ ] **Assumptions:**
    - Orders should only be placed if users have sufficient balances.
    - You do not need to match or execute orders yet. The focus is on order placement and management in memory.

### Bonus Challenge (Optional):

- Implement a basic matching engine that pairs buy and sell orders if their prices meet. When an order is matched, update the balances accordingly.
- Add a websocket layer where user can subscribe to the orderbook for a market for depth updates 