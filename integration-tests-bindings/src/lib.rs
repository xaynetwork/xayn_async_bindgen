// Copyright 2022 Xayn AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Rust bindings used by the integration tests.
#![deny(
    clippy::pedantic,
    clippy::future_not_send,
    clippy::missing_errors_doc,
    noop_method_call,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused_qualifications,
    unsafe_op_in_unsafe_fn
)]
#![warn(missing_docs, unreachable_pub)]
#![allow(
    clippy::must_use_candidate,
    clippy::items_after_statements,
    clippy::module_name_repetitions
)]

use std::mem::forget;

pub mod async_bindings;

#[async_bindgen::api(
    // Imports here must be absolute.
    use std::ffi::c_void;
)]
impl AsyncApi {
    /// Adds two bytes.
    pub async fn add(x: u8, y: u8) -> u8 {
        x + y
    }

    /// Subs two bytes.
    pub async fn sub(x: u8, y: u8) -> u8 {
        x - y
    }

    /// Does nothing, leaks the box.
    pub async fn foo(bar: Box<c_void>) {
        forget(bar);
    }
}

#[async_bindgen::api]
impl Api2 {
    /// Returns 12.
    pub async fn get_the_byte() -> u8 {
        12
    }
}
