// Copyright 2021 Xayn AG
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

#![doc(hidden)]
//! Components used by the bindings auto generated by `async-bindgen-derive` for binding to dart.

use std::future::Future;

use xayn_dart_api_dl::{
    cobject::{CObject, TypedData},
    ports::SendPort,
    DartRuntime,
};

use thiserror::Error;

pub use xayn_dart_api_dl::{initialize_dart_api_dl, ports::DartPortId};

/// An id indicating which future will be completed.
pub type CompleterId = i64;
/// A magic way to pass back results (currently this is a pointer turned into an int).
pub type Handle = i64;

/// Bundled all the additional dart-language specific async call parameters with functionality to do an async call.
pub struct PreparedCompleter {
    send_port: Option<SendPort>,
    completer_id: CompleterId,
}

/// Magic tag acting as a fail safe,
///
/// was randomly set to a number not representable as `f64`.
const MAGIC_TAG: i64 = -6_504_203_682_518_908_873;

impl PreparedCompleter {
    /// Bundle the dart-language specific async call parameters.
    ///
    /// The port id is the port the result must be send back through.
    ///
    /// The completer id helps the ports message handle to complete the
    /// right future.
    ///
    /// # Errors
    ///
    /// Fails if
    /// - the dart runtime was not initialized
    /// - a `DartPortId` with a value of `0` is passed in
    ///
    pub fn new(
        port_id: DartPortId,
        completer_id: CompleterId,
    ) -> Result<Self, CompleterSetupFailed> {
        //TODO log err
        let rt = DartRuntime::instance().map_err(|_| CompleterSetupFailed)?;
        let send_port = rt.send_port_from_raw(port_id).ok_or(CompleterSetupFailed)?;
        Ok(Self {
            send_port: Some(send_port),
            completer_id,
        })
    }

    /// Extract the result of the async call given a handler id.
    ///
    /// # Safety
    ///
    /// - This must only be called with a handler id sent to the
    ///   `FfiCompleterRegistry` which did setup the async call.
    /// - This must only be called once for each handle.
    pub unsafe fn extract_result<T>(handle: Handle) -> T {
        unsafe { decode_box_pointer(handle) }
    }

    /// Spawns the given async-fn's future.
    ///
    /// Once it completes dart is notified about the result.
    ///
    /// If it gets canceled (dropped without completion) the future corresponding
    /// to it in dart is completed with an error.
    pub fn spawn<T: 'static>(self, future: impl Future<Output = T> + Send + 'static) {
        spawn(self.bind_future(future));
    }

    async fn bind_future<T>(mut self, future: impl Future<Output = T> + Send + 'static) {
        let output = future.await;
        let handle = encode_box_pointer(output);
        self.send_result_if_not_already_done(Some(handle));
    }

    /// Sends the result
    ///
    /// Does nothing if the result was already sent.
    fn send_result_if_not_already_done(&mut self, handle: Option<Handle>) {
        if let Some(port) = self.send_port.take() {
            let res = if let Some(handle) = handle {
                CObject::typed_data(TypedData::Int64(vec![MAGIC_TAG, self.completer_id, handle]))
            } else {
                CObject::array(vec![
                    Box::new(CObject::int64(MAGIC_TAG)),
                    Box::new(CObject::int64(self.completer_id)),
                    Box::new(CObject::string_lossy("future canceled in rust")),
                ])
            };
            if let Err(_err) = port.post_cobject(res) {
                //TODO report to error control port
                //IF we do so output result and handle
                //it in spawn?
            }
        }
    }
}

impl Drop for PreparedCompleter {
    fn drop(&mut self) {
        self.send_result_if_not_already_done(None);
    }
}

//TODO enum
/// Setting up the completer failed.
#[derive(Debug, Error)]
#[error("Setting up the completer failed.")]
pub struct CompleterSetupFailed;

/// Spawns the future.
///
// In the future this should allow different implementations (potentially
// with a cfg, for now only async-std as it's easier to use).
fn spawn(future: impl Future<Output = ()> + Send + 'static) {
    // noticeable more complex with tokio as:
    // - We need a handle to the runtime, but are not in the runtime
    //  - So we need to store the handle in some global slot and
    //    have an init runtime function.
    async_std::task::spawn(future);
}

/// Undos what `encode_box_pointer` does.
///
/// # Safety
///
/// This is only safe if
///
/// - the encoded pointer was created by [`encode_box_pointer()`]
/// - it was not done before
unsafe extern "C" fn decode_box_pointer<T>(encoded_box_pointer: i64) -> T {
    #![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    // only ok if size ptr <= size usize <= size isize <= size i64
    let ptr = encoded_box_pointer as isize as usize as *mut T;
    let boxed = unsafe { Box::from_raw(ptr) };
    *boxed
}

fn encode_box_pointer<T>(val: T) -> i64 {
    #![allow(clippy::cast_possible_wrap)]
    // only ok if size ptr <= size usize <= size isize <= size i64
    // for now this is guaranteed on all platforms rust supports,
    // we also added a test to be sure
    Box::into_raw(Box::new(val)) as usize as isize as i64
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn test_usize_u64_and_ptr_size_match() {
        assert!(size_of::<usize>() >= size_of::<&u8>());
        assert!(size_of::<i64>() >= size_of::<usize>());
    }
}
