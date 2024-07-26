//! Stylus Integer Return Types Test Contract

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_primitives::Uint;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{I8, U256, U8},
    prelude::*,
};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint8 unsigned8;
        uint16 unsigned16;
        uint24 unsigned24;
        uint32 unsigned32;
        uint64 unsigned64;
        uint128 unsigned128;
        uint160 unsigned160;
        uint200 unsigned200;
        uint256 unsigned256;
        int8 signed8;
        int16 signed16;
        int24 signed24;
        int32 signed32;
        int64 signed64;
        int128 signed128;
        int160 signed160;
        int200 signed200;
        int256 signed256;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[external]
impl Counter {
    pub fn get_u8(&self) -> U8 {
        self.unsigned8.get()
    }

    pub fn get_u8_builtin(&self) -> u8 {
        self.unsigned8.get().try_into().unwrap()
    }

    pub fn set_u8(&mut self, value: U8) {
        self.unsigned8.set(value)
    }

    pub fn set_u8_builtin(&mut self, value: u8) {
        self.unsigned8.set(value.try_into().unwrap())
    }

    pub fn increment_u8(&mut self) {
        let current = self.unsigned8.get();
        self.unsigned8.set(current + Uint::from(1));
    }
}
