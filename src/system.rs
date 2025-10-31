use std::collections::BTreeMap;

type AccountId = String;
type Nonce = u32;
type BlockNumber = u32;

#[derive(Debug)]
pub struct Pallet {
	/// The current block number.
	block_number: BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		/* TODO: Increment the current block number by one. */
		self.block_number += 1;
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountId) {
		let nonce: Nonce = *self.nonce.get(who).unwrap_or(&0);
		let new_nonce = nonce + 1;
		self.nonce.insert(who.clone(), new_nonce);
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
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
