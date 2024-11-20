use guest_anonymous_account::accounts_hash::{
    calculate_hash_for_accounts, password_hash_fn, AccountData,
};
use risc0_zkvm::guest::env;
use subxt_signer::sr25519;
use subxt_signer::sr25519::{PublicKey, Signature};

fn main() {
    let account_data: AccountData = env::read();

    let account_addresses = account_data.account_addresses.clone();

    let hash = calculate_hash_for_accounts(&account_addresses);
    let current_hash = account_data.current_hash;
    assert_eq!(current_hash, hash);

    let public_key_of_account = PublicKey(account_data.public_key_of_account);

    let public_key_of_account2 = PublicKey(account_data.public_key_of_account);

    // Ensure the Vec<u8> has exactly 64 bytes and convert it to [u8; 64]
    let signature_array: [u8; 64] = account_data.signature.0;

    // Create a Signature from the [u8; 64] array
    let signature_of_account = Signature(signature_array);

    let your_account_id = public_key_of_account.to_account_id();

    // // let mnemonic = Mnemonic::parse(&phrase_of_index).unwrap();
    // // let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
    // // let account_address_of_your_account = keypair.public_key().to_account_id();

    assert_eq!(account_addresses[account_data.index].0, your_account_id);
    let password = account_data.password.as_bytes();
    let password_string = account_data.password.clone();
    assert!(sr25519::verify(
        &signature_of_account,
        password,
        &public_key_of_account2
    ));

    let password_hash = password_hash_fn(account_data.index, password_string);

    // write public output to the journal
    // env::commit(&(current_hash, password_hash));

    env::commit(&(hash, password_hash));
}
