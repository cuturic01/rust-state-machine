use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
	/// The current block number.
    block_number: u32,
	/// A map from an account to their nonce.
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { 
            block_number: 0, 
            nonce: BTreeMap::new()  
        }
	}

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn nonce(&self, who: &String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }
 
    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, who: &String) {
        let mut nonce = self.nonce(who);
        nonce += 1;
        self.nonce.insert(who.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
        let mut system = super::Pallet::new();

        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
        assert_eq!(system.nonce(&"alice".to_string()), 1);
        assert_eq!(system.nonce(&"bob".to_string()), 0);
	}
}