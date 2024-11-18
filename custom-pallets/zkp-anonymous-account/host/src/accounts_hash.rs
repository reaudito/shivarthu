use serde::{Deserialize, Serialize};
use sp_io::hashing::blake2_256;
use subxt_core::utils::AccountId32;
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

#[derive(Serialize, Deserialize)]
pub struct AccountData {
    pub account_addresses: Vec<AccountId32>,
    pub current_hash: [u8; 32],
    pub index: usize,
    pub public_key_of_account: [u8; 32],
    pub signature: Vec<u8>, // Use Vec<u8> instead of [u8; 64]
    pub password: String,
}

fn update_hash_incrementally(current_hash: [u8; 32], account_id: &AccountId32) -> [u8; 32] {
    let mut input_data = Vec::new();

    // Extend input data with the current hash and the new account ID
    input_data.extend_from_slice(&current_hash);
    input_data.extend_from_slice(account_id.as_ref());

    // Recalculate the hash with the new account ID
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
    let mut current_hash: [u8; 32] = [0; 32];
    for phrase in phrases {
        let mnemonic = Mnemonic::parse(phrase).unwrap();
        let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
        let account_address = keypair.public_key().to_account_id();
        println!("{:?}", account_address);
        account_addresses.push(account_address.clone());
        current_hash = update_hash_incrementally(current_hash, &account_address);
    }

    println!("current_hash:{:?}", current_hash);

    let index = 2;

    let phrase_of_index = phrases_clone[index];
    let mnemonic = Mnemonic::parse(phrase_of_index).unwrap();
    let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();

    let public_key_of_account = keypair.public_key().0;

    let password = "password-signature".to_owned();

    let signature = keypair.sign(password.as_bytes());

    // Make the signature public in blockchain, so that person can't enter another password
    let signature_array = signature.0.to_vec();

    AccountData {
        account_addresses: account_addresses,
        current_hash: current_hash,
        index: index,
        public_key_of_account: public_key_of_account,
        signature: signature_array,
        password: password,
    }
}
