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
    types::{Address, I256, U256},
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
            function getI8() external view returns (int8)
            function getI8Builtin() external view returns (int8)
            function setI8(int8 value) external
            function setI8Builtin(int8 value) external
            function incrementI8() external

            function getI16() external view returns (int16)
            function getI16Builtin() external view returns (int16)
            function setI16(int16 value) external
            function setI16Builtin(int16 value) external
            function incrementI16() external

            function getI24() external view returns (int24)
            function getI24Builtin() external view returns (int24)
            function setI24(int24 value) external
            function setI24Builtin(int24 value) external
            function incrementI24() external

            function getI32() external view returns (int32)
            function getI32Builtin() external view returns (int32)
            function setI32(int32 value) external
            function setI32Builtin(int32 value) external
            function incrementI32() external

            function getI64() external view returns (int64)
            function getI64Builtin() external view returns (int64)
            function setI64(int64 value) external
            function setI64Builtin(int64 value) external
            function incrementI64() external

            function getI128() external view returns (int128)
            function getI128Builtin() external view returns (int128)
            function setI128(int128 value) external
            function setI128Builtin(int128 value) external
            function incrementI128() external

            function getI160() external view returns (int160)
            function setI160(int160 value) external
            function incrementI160() external

            function getI200() external view returns (int200)
            function setI200(int200 value) external
            function incrementI200() external

            function getI256() external view returns (int256)
            function setI256(int256 value) external
            function incrementI256() external

            function getU8() external view returns (uint8)
            function getU8Builtin() external view returns (uint8)
            function setU8(uint8 value) external
            function setU8Builtin(uint8 value) external
            function incrementU8() external

            function getU16() external view returns (uint16)
            function getU16Builtin() external view returns (uint16)
            function setU16(uint16 value) external
            function setU16Builtin(uint16 value) external
            function incrementU16() external

            function getU24() external view returns (uint24)
            function getU24Builtin() external view returns (uint24)
            function setU24(uint24 value) external
            function setU24Builtin(uint24 value) external
            function incrementU24() external

            function getU32() external view returns (uint32)
            function getU32Builtin() external view returns (uint32)
            function setU32(uint32 value) external
            function setU32Builtin(uint32 value) external
            function incrementU32() external

            function getU64() external view returns (uint64)
            function getU64Builtin() external view returns (uint64)
            function setU64(uint64 value) external
            function setU64Builtin(uint64 value) external
            function incrementU64() external

            function getU128() external view returns (uint128)
            function getU128Builtin() external view returns (uint128)
            function setU128(uint128 value) external
            function setU128Builtin(uint128 value) external
            function incrementU128() external

            function getU160() external view returns (uint160)
            function setU160(uint160 value) external
            function incrementU160() external

            function getU200() external view returns (uint200)
            function setU200(uint200 value) external
            function incrementU200() external

            function getU256() external view returns (uint256)
            function setU256(uint256 value) external
            function incrementU256() external
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

    // int8
    let pending = counter.set_i8(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set I8 counter via a tx");
    let signed8 = counter.get_i8().call().await;
    println!("Counter I8 value = {:?}", signed8);
    assert_eq!(10, signed8.unwrap());

    let pending = counter.set_i8(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set i8 counter via a tx");
    let signed8builtin = counter.get_i8_builtin().call().await;
    println!("Counter i8 value = {:?}", signed8builtin);
    assert_eq!(100, signed8builtin.unwrap());

    let pending = counter.increment_i8();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented I8 counter via a tx");

    let signed8 = counter.get_i8().call().await;
    println!("New counter I8 value = {:?}", signed8);
    assert_eq!(101, signed8.unwrap());
    let signed8builtin = counter.get_i8_builtin().call().await;
    println!("New Counter i8 value = {:?}", signed8builtin);
    assert_eq!(101, signed8builtin.unwrap());

    // int16
    let pending = counter.set_i16(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set I16 counter via a tx");
    let signed16 = counter.get_i16().call().await;
    println!("Counter I16 value = {:?}", signed16);
    assert_eq!(10, signed16.unwrap());

    let pending = counter.set_i16(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set i16 counter via a tx");
    let signed16builtin = counter.get_i16_builtin().call().await;
    println!("Counter i16 value = {:?}", signed16builtin);
    assert_eq!(100, signed16builtin.unwrap());

    let pending = counter.increment_i16();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented I16 counter via a tx");

    let signed16 = counter.get_i16().call().await;
    println!("New counter I16 value = {:?}", signed16);
    assert_eq!(101, signed16.unwrap());
    let signed16builtin = counter.get_i16_builtin().call().await;
    println!("New Counter i16 value = {:?}", signed16builtin);
    assert_eq!(101, signed16builtin.unwrap());

    // int24
    let pending = counter.set_i24(10.into());
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set I24 counter via a tx");
    let signed24 = counter.get_i24().call().await;
    println!("Counter I24 value = {:?}", signed24);
    assert_eq!(10, signed24.unwrap());

    let pending = counter.increment_i24();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented I24 counter via a tx");

    let signed24 = counter.get_i24().call().await;
    println!("New counter I24 value = {:?}", signed24);
    assert_eq!(11, signed24.unwrap());

    // int32
    let pending = counter.set_i32(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set I32 counter via a tx");
    let signed32 = counter.get_i32().call().await;
    println!("Counter I32 value = {:?}", signed32);
    assert_eq!(10, signed32.unwrap());

    let pending = counter.set_i32(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set i32 counter via a tx");
    let signed32builtin = counter.get_i32_builtin().call().await;
    println!("Counter i32 value = {:?}", signed32builtin);
    assert_eq!(100, signed32builtin.unwrap());

    let pending = counter.increment_i32();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented I32 counter via a tx");

    let signed32 = counter.get_i32().call().await;
    println!("New counter I32 value = {:?}", signed32);
    assert_eq!(101, signed32.unwrap());
    let signed32builtin = counter.get_i32_builtin().call().await;
    println!("New Counter i32 value = {:?}", signed32builtin);
    assert_eq!(101, signed32builtin.unwrap());

    // int64
    let pending = counter.set_i64(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set I64 counter via a tx");
    let signed64 = counter.get_i64().call().await;
    println!("Counter I64 value = {:?}", signed64);
    assert_eq!(10, signed64.unwrap());

    let pending = counter.set_i64(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set i64 counter via a tx");
    let signed64builtin = counter.get_i64_builtin().call().await;
    println!("Counter i64 value = {:?}", signed64builtin);
    assert_eq!(100, signed64builtin.unwrap());

    let pending = counter.increment_i64();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented I64 counter via a tx");

    let signed64 = counter.get_i64().call().await;
    println!("New counter I64 value = {:?}", signed64);
    assert_eq!(101, signed64.unwrap());
    let signed64builtin = counter.get_i64_builtin().call().await;
    println!("New Counter i64 value = {:?}", signed64builtin);
    assert_eq!(101, signed64builtin.unwrap());

    // int128
    let pending = counter.set_i128(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set I128 counter via a tx");
    let signed128 = counter.get_i128().call().await;
    println!("Counter I128 value = {:?}", signed128);
    assert_eq!(10, signed128.unwrap());

    let pending = counter.set_i128(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set i128 counter via a tx");
    let signed128builtin = counter.get_i128_builtin().call().await;
    println!("Counter i128 value = {:?}", signed128builtin);
    assert_eq!(100, signed128builtin.unwrap());

    let pending = counter.increment_i128();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented I128 counter via a tx");

    let signed128 = counter.get_i128().call().await;
    println!("New counter I128 value = {:?}", signed128);
    assert_eq!(101, signed128.unwrap());
    let signed128builtin = counter.get_i128_builtin().call().await;
    println!("New Counter i128 value = {:?}", signed128builtin);
    assert_eq!(101, signed128builtin.unwrap());

    // int160
    let pending = counter.set_i160(10.into());
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set I160 counter via a tx");
    let signed160 = counter.get_i160().call().await;
    println!("Counter I160 value = {:?}", signed160);
    assert_eq!(I256::from(10), signed160.unwrap());

    let pending = counter.increment_i160();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented I160 counter via a tx");

    let signed160 = counter.get_i160().call().await;
    println!("New counter I160 value = {:?}", signed160);
    assert_eq!(I256::from(11), signed160.unwrap());

    // int200
    let pending = counter.set_i200(10.into());
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set I200 counter via a tx");
    let signed200 = counter.get_i200().call().await;
    println!("Counter I200 value = {:?}", signed200);
    assert_eq!(I256::from(10), signed200.unwrap());

    let pending = counter.increment_i200();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented I200 counter via a tx");

    let signed200 = counter.get_i200().call().await;
    println!("New counter I200 value = {:?}", signed200);
    assert_eq!(I256::from(11), signed200.unwrap());

    // int256
    let pending = counter.set_i256(10.into());
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set I256 counter via a tx");
    let signed256 = counter.get_i256().call().await;
    println!("Counter I256 value = {:?}", signed256);
    assert_eq!(I256::from(10), signed256.unwrap());

    let pending = counter.increment_i256();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented I256 counter via a tx");

    let signed256 = counter.get_i256().call().await;
    println!("New counter I256 value = {:?}", signed256);
    assert_eq!(I256::from(11), signed256.unwrap());

    // uint8
    let pending = counter.set_u8(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set U8 counter via a tx");
    let unsigned8 = counter.get_u8().call().await;
    println!("Counter U8 value = {:?}", unsigned8);
    assert_eq!(10, unsigned8.unwrap());

    let pending = counter.set_u8(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set u8 counter via a tx");
    let unsigned8builtin = counter.get_u8_builtin().call().await;
    println!("Counter u8 value = {:?}", unsigned8builtin);
    assert_eq!(100, unsigned8builtin.unwrap());

    let pending = counter.increment_u8();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U8 counter via a tx");

    let unsigned8 = counter.get_u8().call().await;
    println!("New counter U8 value = {:?}", unsigned8);
    assert_eq!(101, unsigned8.unwrap());
    let unsigned8builtin = counter.get_u8_builtin().call().await;
    println!("New Counter u8 value = {:?}", unsigned8builtin);
    assert_eq!(101, unsigned8builtin.unwrap());

    // uint16
    let pending = counter.set_u16(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set U16 counter via a tx");
    let unsigned16 = counter.get_u16().call().await;
    println!("Counter U16 value = {:?}", unsigned16);
    assert_eq!(10, unsigned16.unwrap());

    let pending = counter.set_u16(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set u16 counter via a tx");
    let unsigned16builtin = counter.get_u16_builtin().call().await;
    println!("Counter u16 value = {:?}", unsigned16builtin);
    assert_eq!(100, unsigned16builtin.unwrap());

    let pending = counter.increment_u16();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U16 counter via a tx");

    let unsigned16 = counter.get_u16().call().await;
    println!("New counter U16 value = {:?}", unsigned16);
    assert_eq!(101, unsigned16.unwrap());
    let unsigned16builtin = counter.get_u16_builtin().call().await;
    println!("New Counter u16 value = {:?}", unsigned16builtin);
    assert_eq!(101, unsigned16builtin.unwrap());

    // uint24
    let pending = counter.set_u24(10u32.into());
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set U24 counter via a tx");
    let unsigned24 = counter.get_u24().call().await;
    println!("Counter U24 value = {:?}", unsigned24);
    assert_eq!(10, unsigned24.unwrap());

    let pending = counter.increment_u24();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U24 counter via a tx");

    let unsigned24 = counter.get_u24().call().await;
    println!("New counter U24 value = {:?}", unsigned24);
    assert_eq!(11, unsigned24.unwrap());

    // uint32
    let pending = counter.set_u32(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set U32 counter via a tx");
    let unsigned32 = counter.get_u32().call().await;
    println!("Counter U32 value = {:?}", unsigned32);
    assert_eq!(10, unsigned32.unwrap());

    let pending = counter.set_u32(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set u32 counter via a tx");
    let unsigned32builtin = counter.get_u32_builtin().call().await;
    println!("Counter u32 value = {:?}", unsigned32builtin);
    assert_eq!(100, unsigned32builtin.unwrap());

    let pending = counter.increment_u32();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U32 counter via a tx");

    let unsigned32 = counter.get_u32().call().await;
    println!("New counter U32 value = {:?}", unsigned32);
    assert_eq!(101, unsigned32.unwrap());
    let unsigned32builtin = counter.get_u32_builtin().call().await;
    println!("New Counter u32 value = {:?}", unsigned32builtin);
    assert_eq!(101, unsigned32builtin.unwrap());

    // uint64
    let pending = counter.set_u64(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set U64 counter via a tx");
    let unsigned64 = counter.get_u64().call().await;
    println!("Counter U64 value = {:?}", unsigned64);
    assert_eq!(10, unsigned64.unwrap());

    let pending = counter.set_u64(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set u64 counter via a tx");
    let unsigned64builtin = counter.get_u64_builtin().call().await;
    println!("Counter u64 value = {:?}", unsigned64builtin);
    assert_eq!(100, unsigned64builtin.unwrap());

    let pending = counter.increment_u64();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U64 counter via a tx");

    let unsigned64 = counter.get_u64().call().await;
    println!("New counter U64 value = {:?}", unsigned64);
    assert_eq!(101, unsigned64.unwrap());
    let unsigned64builtin = counter.get_u64_builtin().call().await;
    println!("New Counter u64 value = {:?}", unsigned64builtin);
    assert_eq!(101, unsigned64builtin.unwrap());

    // uint128
    let pending = counter.set_u128(10);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set U128 counter via a tx");
    let unsigned128 = counter.get_u128().call().await;
    println!("Counter U128 value = {:?}", unsigned128);
    assert_eq!(10, unsigned128.unwrap());

    let pending = counter.set_u128(100);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set u128 counter via a tx");
    let unsigned128builtin = counter.get_u128_builtin().call().await;
    println!("Counter u128 value = {:?}", unsigned128builtin);
    assert_eq!(100, unsigned128builtin.unwrap());

    let pending = counter.increment_u128();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U128 counter via a tx");

    let unsigned128 = counter.get_u128().call().await;
    println!("New counter U128 value = {:?}", unsigned128);
    assert_eq!(101, unsigned128.unwrap());
    let unsigned128builtin = counter.get_u128_builtin().call().await;
    println!("New Counter u128 value = {:?}", unsigned128builtin);
    assert_eq!(101, unsigned128builtin.unwrap());

    // uint160
    let pending = counter.set_u160(10.into());
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set U160 counter via a tx");
    let unsigned160 = counter.get_u160().call().await;
    println!("Counter U160 value = {:?}", unsigned160);
    assert_eq!(U256::from(10), unsigned160.unwrap());

    let pending = counter.increment_u160();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U160 counter via a tx");

    let unsigned160 = counter.get_u160().call().await;
    println!("New counter U160 value = {:?}", unsigned160);
    assert_eq!(U256::from(11), unsigned160.unwrap());

    // uint200
    let pending = counter.set_u200(10.into());
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set U200 counter via a tx");
    let unsigned200 = counter.get_u200().call().await;
    println!("Counter U200 value = {:?}", unsigned200);
    assert_eq!(U256::from(10), unsigned200.unwrap());

    let pending = counter.increment_u200();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U200 counter via a tx");

    let unsigned200 = counter.get_u200().call().await;
    println!("New counter U200 value = {:?}", unsigned200);
    assert_eq!(U256::from(11), unsigned200.unwrap());

    // uint256
    let pending = counter.set_u256(10.into());
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully set U256 counter via a tx");
    let unsigned256 = counter.get_u256().call().await;
    println!("Counter U256 value = {:?}", unsigned256);
    assert_eq!(U256::from(10), unsigned256.unwrap());

    let pending = counter.increment_u256();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}", receipt);
    }
    println!("Successfully incremented U256 counter via a tx");

    let unsigned256 = counter.get_u256().call().await;
    println!("New counter U256 value = {:?}", unsigned256);
    assert_eq!(U256::from(11), unsigned256.unwrap());

    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}
