//! Example on how to interact with a deployed `int-return-types` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. This is repeated for several different integer types.
//! The deployed program is fully written in Rust and compiled to WASM but with Stylus,
//! it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::eyre;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

/// Your private key file path.
const PRIV_KEY_PATH: &str = "PRIV_KEY_PATH";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

/// Deployed pragram address.
const STYLUS_PROGRAM_ADDRESS: &str = "ADDR";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let priv_key_path =
        std::env::var(PRIV_KEY_PATH).map_err(|_| eyre!("No {} env var set", PRIV_KEY_PATH))?;
    let rpc_url = std::env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    let program_address = std::env::var(STYLUS_PROGRAM_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", STYLUS_PROGRAM_ADDRESS))?;
    abigen!(
        Counter,
        r#"[
            function getU8() external view returns (uint8)
            function getU8Builtin() external view returns (uint8)
            function setU8(uint8 value) external
            function setU8Builtin(uint8 value) external
            function incrementU8() external
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let privkey = read_secret_from_file(&priv_key_path)?;
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let counter = Counter::new(address, client);

    let unsigned8 = counter.get_u8().call().await;
    println!("Counter U8 value = {:?}", unsigned8);

    let pending = counter.increment_u8();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U8 counter via a tx");

    let unsigned8 = counter.get_u8().call().await;
    println!("New counter number value = {:?}", unsigned8);
    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}
