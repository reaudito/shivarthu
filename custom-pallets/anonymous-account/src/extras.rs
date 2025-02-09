use super::*;

use parity_scale_codec::Encode;
use scale_info::prelude::vec::Vec;
use sp_io::hashing::blake2_256;
use sp_io::hashing::sha2_256;

impl<T: Config> Pallet<T> {
	pub fn update_hash_incrementally(current_hash: [u8; 32], account_id: Vec<u8>) -> [u8; 32] {
		let mut input_data = Vec::new();

		// Extend input data with the current hash and the new account ID
		input_data.extend_from_slice(&current_hash);
		input_data.extend_from_slice(account_id.as_ref());

		// Recalculate the hash with the new account ID
		blake2_256(&input_data)
	}

	pub fn calculate_hash_for_accounts2(accounts: &[(T::AccountId, [u8; 64])]) -> [u8; 32] {
		let mut input_data = Vec::new();

		// Concatenate all account IDs into a single byte vector
		for account in accounts {
			let encoded_id = account.0.encode();
			input_data.extend_from_slice(encoded_id.as_ref());
			input_data.extend_from_slice(account.1.as_ref());
		}

		// Compute the hash of the combined data
		blake2_256(&input_data)
	}

	pub fn calculate_hash_for_accounts(accounts: &[(T::AccountId, [u8; 64])]) -> [u8; 32] {
		let mut input_data = Vec::new();

		// Concatenate all account IDs into a single byte vector
		for account in accounts {
			let encoded_id = account.0.encode();
			input_data.extend_from_slice(encoded_id.as_ref());
			input_data.extend_from_slice(account.1.as_ref());
		}

		// Compute the hash of the combined data
		sha2_256(&input_data)
	}

	pub fn get_slice_hash(
		department_id: DepartmentId,
		slice_number: u32,
	) -> Result<[u8; 32], DispatchError> {
		KYCHashes::<T>::get(department_id, slice_number).ok_or(Error::<T>::HashNotFound.into())
	}
}
