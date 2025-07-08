use std::{collections::BTreeMap, ops::Add};

use num::{One, Zero};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + Add + Copy;
    type Nonce: Zero + One + Add + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
    block_number: T::BlockNumber,
	/// A map from an account to their nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T>
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { 
            block_number: T::BlockNumber::zero(), 
            nonce: BTreeMap::new()  
        }
	}

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
 
    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number + T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let mut nonce = self.nonce(who);
        nonce = nonce + T::Nonce::one();
        self.nonce.insert(who.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
        let mut system = super::Pallet::<TestConfig>::new();

        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
        assert_eq!(system.nonce(&"alice".to_string()), 1);
        assert_eq!(system.nonce(&"bob".to_string()), 0);
	}
}