use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use sp_io::hashing::blake2_256;
use subxt_core::utils::AccountId32;

#[derive(Serialize, Deserialize, Clone)]
pub struct ByteArray64(#[serde(with = "BigArray")] pub [u8; 64]);

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountData {
    pub account_addresses: Vec<(AccountId32, ByteArray64)>,
    pub current_hash: [u8; 32],
    pub index: usize,
    pub public_key_of_account: [u8; 32],
    pub signature: ByteArray64, // Use Vec<u8> instead of [u8; 64]
    pub password: String,
}

pub fn calculate_hash_for_accounts(accounts: &[(AccountId32, ByteArray64)]) -> [u8; 32] {
    let mut input_data = Vec::new();

    // Concatenate all account IDs and ByteArray64 contents into a single byte vector
    for account in accounts {
        input_data.extend_from_slice(account.0.as_ref()); // AccountId32 as bytes
        input_data.extend_from_slice(&(account.1).0); // ByteArray64's inner array
    }

    // Compute the hash of the combined data
    blake2_256(&input_data)
}
pub fn password_hash_fn(index: usize, password: String) -> [u8; 32] {
    // Convert the index to bytes (using 8 bytes to represent usize)
    let mut index_bytes = index.to_le_bytes().to_vec();

    // Convert the password string to bytes and append to index bytes
    index_bytes.extend(password.as_bytes());
    // Hash the combined data using blake2_256
    blake2_256(&index_bytes)
}
