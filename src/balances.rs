use std::collections::BTreeMap;

pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balances(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(
		&mut self,
		from: &String,
		to: &String,
		amount: u128,
	) -> Result<(), &'static str> {
		let from_balance = self.balance(from);
		let to_balance = self.balance(to);

		let from_balance = from_balance
            .checked_sub(amount)
            .ok_or("Not enough funds.")?;

		let to_balance = to_balance
            .checked_add(amount)
            .ok_or("Owerflow occured.")?;

		self.set_balances(from, from_balance);
		self.set_balances(to, to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
    
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balances(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

    #[test]
    fn transfer_funds() {
        let mut balances = super::Pallet::new();
        
        balances.set_balances(&"alice".to_string(), 100);
        balances.set_balances(&"bob".to_string(), 100);

        let result = balances
            .transfer(
                &"alice".to_string(), 
                &"bob".to_string(), 
                50
        );

        assert_eq!(result, Ok(()));
        assert_eq!(balances.balance(&"alice".to_string()), 50);
		assert_eq!(balances.balance(&"bob".to_string()), 150);
    }
}
