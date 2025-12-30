// Copyright 2023-2024, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/main/licenses/COPYRIGHT.md

#![no_std]
#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(target_family = "wasm"))] {
        mod imp;
        pub use imp::MiniAlloc;
    }
}
