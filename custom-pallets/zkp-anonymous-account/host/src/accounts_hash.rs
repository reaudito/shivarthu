use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use sp_io::hashing::blake2_256;
use subxt_core::utils::AccountId32;
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

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

pub fn keypair_func() -> AccountData {
    let mut phrases = Vec::new();
    phrases.push("bottom drive obey lake curtain smoke basket hold race lonely fit walk");
    phrases.push("demand toy recycle symptom this arrow pear ribbon orchard large cabin tower");
    phrases.push("repair resist urban upgrade delay security blush vote bean moment current drill");
    phrases
        .push("disagree romance reform wink essence speak sense fence cause reflect sound alarm");
    phrases.push("figure husband please idea captain bulk despair over letter code art mimic");
    phrases.push("regret family similar face thumb magic head team duty web side strike");
    phrases.push("resemble timber picnic stage must video amount price sport help good stable");
    let phrases_clone = phrases.clone();
    let mut account_addresses = Vec::new();
    for phrase in phrases {
        let mnemonic = Mnemonic::parse(phrase).unwrap();
        let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
        let account_address = keypair.public_key().to_account_id();
        println!("{:?}", account_address);

        let password = "password-signature".to_owned();

        let signature = keypair.sign(password.as_bytes());
        account_addresses.push((account_address.clone(), ByteArray64(signature.0)));
    }

    let mut account_addresses_new = Vec::new();
    let copies = 250;

    for _ in 0..copies {
        account_addresses_new.extend(account_addresses.clone());
    }

    let current_hash = calculate_hash_for_accounts(&account_addresses_new.clone());

    let index = 2;

    let phrase_of_index = phrases_clone[index];
    let mnemonic = Mnemonic::parse(phrase_of_index).unwrap();
    let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();

    let public_key_of_account = keypair.public_key().0;

    let password = "password-signature".to_owned();

    let signature = keypair.sign(password.as_bytes());

    // Make the signature public in blockchain, so that person can't enter another password
    let signature_array = signature.0;

    AccountData {
        account_addresses: account_addresses_new,
        current_hash: current_hash,
        index: index,
        public_key_of_account: public_key_of_account,
        signature: ByteArray64(signature_array),
        password: password,
    }
}
