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

// #[rustfmt::skip]
pub mod async_api;
// #[rustfmt::skip]
pub mod api2;

// this normally is included in the proc macro expansion
use crate::async_api::AsyncApi;

// #[async_bindgen::api]
impl AsyncApi {
    /// Adds two bytes.
    pub async fn add(x: u8, y: u8) -> u8 {
        x + y
    }

    /// Subs two bytes.
    pub async fn sub(x: u8, y: u8) -> u8 {
        x - y
    }
}

// this normally is included in the proc macro expansion
use crate::api2::Api2;

// #[async_bindgen::api]
impl Api2 {
    /// Returns 12.
    pub async fn get_the_byte() -> u8 {
        12
    }
}
