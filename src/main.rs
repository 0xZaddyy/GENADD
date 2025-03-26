use bitcoin::network::Network;
use bitcoin::key::{PrivateKey, PublicKey};
use bitcoin::address::Address;
use bitcoin::secp256k1::{Secp256k1, SecretKey};

fn main() {
    // Create a secp256k1 context
    let secp = Secp256k1::new();

    // Generate a random secret key
    let secret_key = SecretKey::new(&mut rand::thread_rng());

    // Create a private key using the secret key
    let private_key = PrivateKey::new(secret_key, Network::Bitcoin);

    // Derive the public key
    let public_key = PublicKey::from_private_key(&secp, &private_key);

    // Generate a Bitcoin address from the public key
    let address = Address::p2pkh(&public_key, Network::Bitcoin);

    // Print the generated Bitcoin address and private key
    println!("Generated Bitcoin Address: {}", address);
    println!("Private Key (WIF format): {}", private_key.to_wif());
}
