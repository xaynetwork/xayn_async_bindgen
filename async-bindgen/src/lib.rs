//! Async extern "C" bindings. Currently limited to dart.
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

pub mod dart;
