//! Stylus Integer Return Types Test Contract

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_primitives::{Signed, Uint};
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{I128, I16, I160, I256, I32, I64, I8, U128, U16, U160, U256, U32, U64, U8},
    prelude::*,
};

type I24 = Signed<24, 1>;
type I200 = Signed<200, 4>;
type U24 = Uint<24, 1>;
type U200 = Uint<200, 4>;

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        int8 signed8;
        int16 signed16;
        int24 signed24;
        int32 signed32;
        int64 signed64;
        int128 signed128;
        int160 signed160;
        int200 signed200;
        int256 signed256;
        uint8 unsigned8;
        uint16 unsigned16;
        uint24 unsigned24;
        uint32 unsigned32;
        uint64 unsigned64;
        uint128 unsigned128;
        uint160 unsigned160;
        uint200 unsigned200;
        uint256 unsigned256;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[external]
impl Counter {
    pub fn get_i8(&self) -> I8 {
        self.signed8.get()
    }

    pub fn get_i8_builtin(&self) -> i8 {
        self.signed8.get().try_into().unwrap()
    }

    pub fn set_i8(&mut self, value: I8) {
        self.signed8.set(value)
    }

    pub fn set_i8_builtin(&mut self, value: i8) {
        self.signed8.set(value.try_into().unwrap())
    }

    pub fn increment_i8(&mut self) {
        let current = self.signed8.get();
        self.signed8.set(current + Signed::try_from(1).unwrap());
    }

    pub fn get_i16(&self) -> I16 {
        self.signed16.get()
    }

    pub fn get_i16_builtin(&self) -> i16 {
        self.signed16.get().try_into().unwrap()
    }

    pub fn set_i16(&mut self, value: I16) {
        self.signed16.set(value)
    }

    pub fn set_i16_builtin(&mut self, value: i16) {
        self.signed16.set(value.try_into().unwrap())
    }

    pub fn increment_i16(&mut self) {
        let current = self.signed16.get();
        self.signed16.set(current + Signed::try_from(1).unwrap());
    }

    pub fn get_i24(&self) -> I24 {
        self.signed24.get()
    }

    pub fn set_i24(&mut self, value: I24) {
        self.signed24.set(value)
    }

    pub fn increment_i24(&mut self) {
        let current = self.signed24.get();
        self.signed24.set(current + Signed::try_from(1).unwrap());
    }

    pub fn get_i32(&self) -> I32 {
        self.signed32.get()
    }

    pub fn get_i32_builtin(&self) -> i32 {
        self.signed32.get().try_into().unwrap()
    }

    pub fn set_i32(&mut self, value: I32) {
        self.signed32.set(value)
    }

    pub fn set_i32_builtin(&mut self, value: i32) {
        self.signed32.set(value.try_into().unwrap())
    }

    pub fn increment_i32(&mut self) {
        let current = self.signed32.get();
        self.signed32.set(current + Signed::try_from(1).unwrap());
    }

    pub fn get_i64(&self) -> I64 {
        self.signed64.get()
    }

    pub fn get_i64_builtin(&self) -> i64 {
        self.signed64.get().try_into().unwrap()
    }

    pub fn set_i64(&mut self, value: I64) {
        self.signed64.set(value)
    }

    pub fn set_i64_builtin(&mut self, value: i64) {
        self.signed64.set(value.try_into().unwrap())
    }

    pub fn increment_i64(&mut self) {
        let current = self.signed64.get();
        self.signed64.set(current + Signed::try_from(1).unwrap());
    }

    pub fn get_i128(&self) -> I128 {
        self.signed128.get()
    }

    pub fn get_i128_builtin(&self) -> i128 {
        self.signed128.get().try_into().unwrap()
    }

    pub fn set_i128(&mut self, value: I128) {
        self.signed128.set(value)
    }

    pub fn set_i128_builtin(&mut self, value: i128) {
        self.signed128.set(value.try_into().unwrap())
    }

    pub fn increment_i128(&mut self) {
        let current = self.signed128.get();
        self.signed128.set(current + Signed::try_from(1).unwrap());
    }

    pub fn get_i160(&self) -> I160 {
        self.signed160.get()
    }

    pub fn set_i160(&mut self, value: I160) {
        self.signed160.set(value)
    }

    pub fn increment_i160(&mut self) {
        let current = self.signed160.get();
        self.signed160.set(current + Signed::try_from(1).unwrap());
    }

    pub fn get_i200(&self) -> I200 {
        self.signed200.get()
    }

    pub fn set_i200(&mut self, value: I200) {
        self.signed200.set(value)
    }

    pub fn increment_i200(&mut self) {
        let current = self.signed200.get();
        self.signed200.set(current + Signed::try_from(1).unwrap());
    }

    pub fn get_i256(&self) -> I256 {
        self.signed256.get()
    }

    pub fn set_i256(&mut self, value: I256) {
        self.signed256.set(value)
    }

    pub fn increment_i256(&mut self) {
        let current = self.signed256.get();
        self.signed256.set(current + Signed::try_from(1).unwrap());
    }

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

    pub fn get_u16(&self) -> U16 {
        self.unsigned16.get()
    }

    pub fn get_u16_builtin(&self) -> u16 {
        self.unsigned16.get().try_into().unwrap()
    }

    pub fn set_u16(&mut self, value: U16) {
        self.unsigned16.set(value)
    }

    pub fn set_u16_builtin(&mut self, value: u16) {
        self.unsigned16.set(value.try_into().unwrap())
    }

    pub fn increment_u16(&mut self) {
        let current = self.unsigned16.get();
        self.unsigned16.set(current + Uint::from(1));
    }

    pub fn get_u24(&self) -> U24 {
        self.unsigned24.get()
    }

    pub fn set_u24(&mut self, value: U24) {
        self.unsigned24.set(value)
    }

    pub fn increment_u24(&mut self) {
        let current = self.unsigned24.get();
        self.unsigned24.set(current + Uint::from(1));
    }

    pub fn get_u32(&self) -> U32 {
        self.unsigned32.get()
    }

    pub fn get_u32_builtin(&self) -> u32 {
        self.unsigned32.get().try_into().unwrap()
    }

    pub fn set_u32(&mut self, value: U32) {
        self.unsigned32.set(value)
    }

    pub fn set_u32_builtin(&mut self, value: u32) {
        self.unsigned32.set(value.try_into().unwrap())
    }

    pub fn increment_u32(&mut self) {
        let current = self.unsigned32.get();
        self.unsigned32.set(current + Uint::from(1));
    }

    pub fn get_u64(&self) -> U64 {
        self.unsigned64.get()
    }

    pub fn get_u64_builtin(&self) -> u64 {
        self.unsigned64.get().try_into().unwrap()
    }

    pub fn set_u64(&mut self, value: U64) {
        self.unsigned64.set(value)
    }

    pub fn set_u64_builtin(&mut self, value: u64) {
        self.unsigned64.set(value.try_into().unwrap())
    }

    pub fn increment_u64(&mut self) {
        let current = self.unsigned64.get();
        self.unsigned64.set(current + Uint::from(1));
    }

    pub fn get_u128(&self) -> U128 {
        self.unsigned128.get()
    }

    pub fn get_u128_builtin(&self) -> u128 {
        self.unsigned128.get().try_into().unwrap()
    }

    pub fn set_u128(&mut self, value: U128) {
        self.unsigned128.set(value)
    }

    pub fn set_u128_builtin(&mut self, value: u128) {
        self.unsigned128.set(value.try_into().unwrap())
    }

    pub fn increment_u128(&mut self) {
        let current = self.unsigned128.get();
        self.unsigned128.set(current + Uint::from(1));
    }

    pub fn get_u160(&self) -> U160 {
        self.unsigned160.get()
    }

    pub fn set_u160(&mut self, value: U160) {
        self.unsigned160.set(value)
    }

    pub fn increment_u160(&mut self) {
        let current = self.unsigned160.get();
        self.unsigned160.set(current + Uint::from(1));
    }

    pub fn get_u200(&self) -> U200 {
        self.unsigned200.get()
    }

    pub fn set_u200(&mut self, value: U200) {
        self.unsigned200.set(value)
    }

    pub fn increment_u200(&mut self) {
        let current = self.unsigned200.get();
        self.unsigned200.set(current + Uint::from(1));
    }

    pub fn get_u256(&self) -> U256 {
        self.unsigned256.get()
    }

    pub fn set_u256(&mut self, value: U256) {
        self.unsigned256.set(value)
    }

    pub fn increment_u256(&mut self) {
        let current = self.unsigned256.get();
        self.unsigned256.set(current + Uint::from(1));
    }
}
