use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Serialize)]
pub struct UserBalance {
    pub balance: u64,
    pub locked: u64,
}

type UserId = String;

#[derive(Serialize)]
pub struct InrBalance {
    user_balances: HashMap<UserId, UserBalance>,
}

impl InrBalance {
    pub fn instance() -> Arc<Mutex<InrBalance>> {
        static mut INSTANCE: Option<Arc<Mutex<InrBalance>>> = None;

        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Arc::new(Mutex::new(InrBalance {
                    user_balances: HashMap::new(),
                })));
            }
            INSTANCE.as_ref().unwrap().clone()
        }
    }

    pub fn user_exists(&self, user_id: &UserId) -> bool {
        self.user_balances.contains_key(user_id)
    }

    // create new user
    pub fn add_user(&mut self, user_id: &UserId) {
        self.user_balances.entry(user_id.to_owned()).or_default();
    }

    // Method to get the balance of a user
    pub fn get_balance(&self, user_id: &UserId) -> Option<&UserBalance> {
        self.user_balances.get(user_id)
    }

    // increase balance
    pub fn onramp_balance(&mut self, user_id: &UserId, amount: u64) -> Result<(), String> {
        if let Some(user_balance) = self.user_balances.get_mut(user_id) {
            user_balance.balance += amount;
            Ok(())
        } else {
            // Optionally handle the case where the user does not exist
            return Err(format!("User {} not found.", user_id));
        }
    }

    // decrease balance
    pub fn deduct_balance(&mut self, user_id: &UserId, amount: u64) -> Result<(), String> {
        if let Some(user_balance) = self.user_balances.get_mut(user_id) {
            if user_balance.balance >= amount {
                user_balance.balance -= amount;
                Ok(())
            } else {
                return Err("Insufficient funds".to_owned());
            }
        } else {
            // Optionally handle the case where the user does not exist
            return Err(format!("User {} not found.", user_id));
        }
    }

    // Lock Balance
    pub fn lock_balance(&mut self, user_id: &UserId, amount: u64) -> Result<(), String> {
        if let Some(user_balance) = self.user_balances.get_mut(user_id) {
            if user_balance.balance >= amount {
                user_balance.balance -= amount;
                user_balance.locked += amount;
                Ok(())
            } else {
                return Err("Insufficient funds".to_owned());
            }
        } else {
            return Err(format!("User {} not found.", user_id));
        }
    }

    // Unlock Balance
    pub fn unlock_balance(&mut self, user_id: &UserId, amount: u64) -> Result<(), String> {
        if let Some(user_balance) = self.user_balances.get_mut(user_id) {
            if user_balance.locked >= amount {
                user_balance.locked -= amount;
                user_balance.balance += amount;
                Ok(())
            } else {
                return Err("Not enough locked balance to unlock".to_owned());
            }
        } else {
            return Err(format!("User {} not found.", user_id));
        }
    }

    // deduct locked balance
    pub fn deduct_locked(&mut self, user_id: &UserId, amount: u64) -> Result<(), String> {
        if let Some(user_balance) = self.user_balances.get_mut(user_id) {
            if user_balance.locked >= amount {
                user_balance.locked -= amount;
                Ok(())
            } else {
                return Err("Insufficient funds".to_owned());
            }
        } else {
            return Err(format!("User {} not found.", user_id));
        }
    }
}

// TODO audit again
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let instance = InrBalance::instance();
        let mut mutex_guard_inr_balance = instance.lock().unwrap();
        let new_user = "user1".to_owned();
        mutex_guard_inr_balance.add_user(&new_user);

        let balance = mutex_guard_inr_balance.get_balance(&new_user).unwrap().balance;
        let locked = mutex_guard_inr_balance.get_balance(&new_user).unwrap().locked;
        let new_user_exits = mutex_guard_inr_balance.user_exists(&new_user);

        assert_eq!(balance, 0);
        assert_eq!(locked, 0);
        assert_eq!(new_user_exits, true)

    }

    #[test]
    fn test_add_balance() {
        let instance = InrBalance::instance();
        let mut mutex_guard_inr_balance = instance.lock().unwrap();
        let new_user = "user2".to_owned();
        mutex_guard_inr_balance.add_user(&new_user);
        let result = mutex_guard_inr_balance.onramp_balance(&new_user, 1000);
        
        let balance = mutex_guard_inr_balance.get_balance(&new_user).unwrap().balance;
        assert_eq!(result.is_ok(), true);
        assert_eq!(balance, 1000);
    }

    #[test]
    fn test_deduct_balance() {
        let instance = InrBalance::instance();
        let mut mutex_guard_inr_balance = instance.lock().unwrap();
        let new_user = "user3".to_owned();
        mutex_guard_inr_balance.add_user(&new_user);
        mutex_guard_inr_balance.onramp_balance(&new_user, 1000).unwrap();
        let result = mutex_guard_inr_balance.deduct_balance(&new_user, 100);
        assert_eq!(result.is_ok(), true);
        let balance = mutex_guard_inr_balance.get_balance(&new_user).unwrap().balance;
        assert_eq!(balance, 900);
    }

    #[test]
    fn test_lock_balance() {
        let instance = InrBalance::instance();
        let mut mutex_guard_inr_balance = instance.lock().unwrap();
        let new_user = "user4".to_owned();
        mutex_guard_inr_balance.add_user(&new_user);
        mutex_guard_inr_balance.onramp_balance(&new_user, 1000).unwrap();
        let result = mutex_guard_inr_balance.lock_balance(&new_user, 100);
        assert_eq!(result.is_ok(), true);
        let balance = mutex_guard_inr_balance.get_balance(&new_user).unwrap().balance;
        let locked = mutex_guard_inr_balance.get_balance(&new_user).unwrap().locked;
        assert_eq!(balance, 900);
        assert_eq!(locked, 100);
    }

    #[test]
    fn test_unlock_balance() {
        let instance = InrBalance::instance();
        let mut mutex_guard_inr_balance = instance.lock().unwrap();
        let new_user = "user5".to_owned();
        mutex_guard_inr_balance.add_user(&new_user);
        mutex_guard_inr_balance.onramp_balance(&new_user, 1000).unwrap();
        mutex_guard_inr_balance.lock_balance(&new_user, 100).unwrap();
        
        let result = mutex_guard_inr_balance.unlock_balance(&new_user, 100);
        assert_eq!(result.is_ok(), true);
        let balance = mutex_guard_inr_balance.get_balance(&new_user).unwrap().balance;
        let locked = mutex_guard_inr_balance.get_balance(&new_user).unwrap().locked;
        assert_eq!(balance, 1000);
        assert_eq!(locked, 0);
    }

    #[test]
    fn test_deduct_locked() {
        let instance = InrBalance::instance();
        let mut mutex_guard_inr_balance = instance.lock().unwrap();
        let new_user = "user6".to_owned();
        mutex_guard_inr_balance.add_user(&new_user);
        mutex_guard_inr_balance.onramp_balance(&new_user, 1000).unwrap();
        mutex_guard_inr_balance.lock_balance(&new_user, 100).unwrap();
        
        let result = mutex_guard_inr_balance.deduct_locked(&new_user, 100);
        assert_eq!(result.is_ok(), true);
        let balance = mutex_guard_inr_balance.get_balance(&new_user).unwrap().balance;
        let locked = mutex_guard_inr_balance.get_balance(&new_user).unwrap().locked;
        assert_eq!(balance, 900);
        assert_eq!(locked, 0);
    }
}
