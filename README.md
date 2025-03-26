# GENADD
Bitcoin Address and Private Key Generator

This Rust project generates Bitcoin private keys and corresponding addresses using a Rust library.

Features

Generates a new Bitcoin private key.

Supports Wallet Import Format (WIF) encoding.

Uses cryptographic libraries for secure key generation.

Prerequisites

Ensure you have the following installed:

Rust and Cargo (Install Rust)

Installation

Clone the repository and navigate into the project directory:

git clone <repository-url>
cd genadd

Usage

Run the generator using Cargo:

cargo run

Example Output

Private Key (WIF):  5J3mBbAH58CERFAi8dUBjSHz4eHEyP3yLBXX8G5zeXCfcD3r2uh
Bitcoin Address:    1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa

Dependencies

This project uses:

bitcoin crate for Bitcoin key management

rand for random number generation

secp256k1 for elliptic curve operations

Ensure these dependencies are included in Cargo.toml:

[dependencies]
bitcoin = "0.29.2"
rand = "0.8"
secp256k1 = { version = "0.24", features = ["rand", "global-context"] }

Security Notes

Keep your private keys secure; never share them.

Do not use generated keys on mainnet unless you understand the risks.


Author

Developed by 0xZaddy
