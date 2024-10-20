use std::collections::HashMap;
use crate::db::UserId;

use super::Users;

#[derive(Clone,)]
pub struct BalanceInfo {
    pub balance: u64,
    pub locked: u64,
}

#[derive(Clone)]
pub struct InrBalance {
    pub user: HashMap<UserId, Option<BalanceInfo>> 
}

impl InrBalance {
    pub fn new(user_id: &String) -> InrBalance {
        let mut new_user = HashMap::new();
        let balance_info: BalanceInfo = BalanceInfo {
            balance: 0,
            locked: 0
        };
        let balance = Some(balance_info);
        new_user.insert(user_id.to_string(), balance);

        // let mut users_vec = Users::new();
        // users_vec.push(user_id.to_owned());

        Self { user: new_user }
    }

    // use this method for onramp
    pub fn add_inr_balance(&mut self, user_id: &String, balance: u64) {
        if let Some(user_balance) = self.user.get_mut(user_id) {
            user_balance.as_mut().unwrap().balance += balance;
        }
    }

    // for locking the balance
    pub fn lock_inr_balance(&mut self, user_id: &String, balance: u64) {
        if let Some(user_balance) = self.user.get_mut(user_id) {
            user_balance.as_mut().unwrap().locked += balance;
            user_balance.as_mut().unwrap().balance -= balance;
        }
    }

    // releasing the locked balance
    pub fn release_lock_inr_balance(&mut self, user_id: &String, balance: u64) {
        match self.user.get_mut(user_id) {
            Some(user_balance) => {
                user_balance.as_mut().unwrap().locked -= balance;
                user_balance.as_mut().unwrap().balance += balance;
            }
            None => println!("user not found")
        }
    }

    // get balance of particular user
    pub fn get_inr_balance(&self, user_id: &String) -> BalanceInfo {
        let balance = self.user.get(user_id).unwrap().as_ref().unwrap().balance;
        let locked = self.user.get(user_id).unwrap().as_ref().unwrap().locked;

        BalanceInfo { balance, locked }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_new_user() {
        let mut inr_balance = InrBalance::new(&"user1".to_owned());
        inr_balance.add_inr_balance(&"user1".to_string(), 100);

        assert_eq!(inr_balance.get_inr_balance(&"user1".to_owned()).balance, 100);
    }

    #[test]
    fn test_add_inr_balance() {
        let mut inr_balance = InrBalance::new(&"user1".to_owned());
        inr_balance.add_inr_balance(&"user1".to_string(), 100);

        inr_balance.add_inr_balance(&"user1".to_owned(), 200);

        assert_eq!(inr_balance.get_inr_balance(&"user1".to_owned()).balance, 300);
    }

    #[test]
    fn test_get_inr_balance() {
        let mut inr_balance = InrBalance::new(&"user1".to_owned());
        inr_balance.add_inr_balance(&"user1".to_string(), 100);

        assert_eq!(inr_balance.get_inr_balance(&"user1".to_owned()).balance, 100);
    }

    #[test]
    fn test_lock_inr_balance() {
        let mut inr_balance = InrBalance::new(&"user1".to_owned());
        inr_balance.add_inr_balance(&"user1".to_string(), 100);

        inr_balance.lock_inr_balance(&"user1".to_owned(), 50);

        assert_eq!(inr_balance.get_inr_balance(&"user1".to_owned()).balance, 50);
        assert_eq!(inr_balance.get_inr_balance(&"user1".to_owned()).locked, 50);
    }   

    #[test]
    fn test_release_lock_inr_balance() {
        let mut inr_balance = InrBalance::new(&"user1".to_owned());
        inr_balance.add_inr_balance(&"user1".to_string(), 100);

        inr_balance.lock_inr_balance(&"user1".to_owned(), 70);

        inr_balance.release_lock_inr_balance(&"user1".to_owned(), 40);

        assert_eq!(inr_balance.get_inr_balance(&"user1".to_owned()).balance, 70);
        assert_eq!(inr_balance.get_inr_balance(&"user1".to_owned()).locked, 30);
    }
}