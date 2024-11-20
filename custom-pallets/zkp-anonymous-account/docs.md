# anonymous-account-crates


**Zero Knowledge Proof for Anonymous Voting**

In a democracy, voting must remain private to ensure the integrity of the process. However, maintaining privacy in blockchain systems is challenging because transactions are recorded on a public ledger, making it difficult to keep accounts anonymous.

**How can zero-knowledge proofs enable anonymous voting?**

**Anonymous Voting Account Creation**

1. Start with 100-200 accounts that have undergone KYC (Know Your Customer) verification.
2. Generate a cryptographic hash for these accounts and store it on the blockchain.
3. Create a signature from your crypto account using a password. Keep the password secret and publish the signature on the blockchain. Publishing the signature does not allow the user to change the password.

**In Risczero:**

1. Take the hash of the accounts and signatures as input.
2. Retrieve the full list of accounts and signatures.
3. Use the hash and cryptographically sign only one account, which will be used to create an anonymous voting account.
4. Push the following to the journal:
   - The newly created anonymous account.
   - The input hash.
   - A total hash counter (e.g., a hash of 0000010000000 combined with the password).

To ensure each anonymous account is unique, the hash counter must also be unique, allowing only one anonymous account per verified identity.

Finally, add the zero-knowledge proof to the blockchain, ensuring that the anonymous voting process is secure and private without revealing the identity of the voter.
