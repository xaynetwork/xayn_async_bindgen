use std::future::Future;

use futures_channel::oneshot::{Canceled, Receiver, Sender, channel};

use crate::opaque::OpaqueData;

#[cfg(feature = "dart-native")]
pub mod dart_native;

// Box<PlatformCallback>
// always created in rust
// but potentially before calling the rust fn

// platform specific implementation
pub trait RustCompletesAsyncCall: Send {
    // any "waiting" is platform specific and "injected" when creating it
    fn complete(self: Box<Self>, data: OpaqueData);
}


impl<F> RustCompletesAsyncCall for F
where
    F: Send + FnOnce(OpaqueData)
{
    fn complete(self: Box<Self>, data: OpaqueData) {
        (self)(data);
    }
}

trait RustAsyncCommand {
    /// Calls the specific async function represented by this command.
    ///
    /// The future produced from the async function must then be feed
    /// with the completer into [`bind_completer_to_future()`] and the
    /// resulting future should be boxed and returned.
    fn call(self: Box<Self>, completer: Box<dyn RustCompletesAsyncCall>) -> Box<dyn Future<Output = ()>>;
}

pub fn bind_completer_to_future(
    future: impl Future<Output = OpaqueData> + 'static,
    completer: Box<dyn RustCompletesAsyncCall>
) -> Box<dyn Future<Output = ()>> {
    Box::new(async move {
        let result = future.await;
        completer.complete(result);
    })
}


#[no_mangle]
pub extern "C" fn extern_completes_async_call(mut extern_callback: Box<ExternCompleter>, data: OpaqueData) -> isize {
    //TODO panic guard
    let res = extern_callback.complete(data);
    //TODO error
    if res.is_ok() {
        0
    } else {
        -1
    }
}


pub struct ExternCompleter {
    sender: Option<Sender<OpaqueData>>
}

impl ExternCompleter {
    pub fn new() -> (Box<Self>, AwaitExtern) {
        let (sender, receiver) = channel();
        let completer = Box::new(ExternCompleter {
            sender: Some(sender)
        });
        let awaiter = AwaitExtern {
            receiver,
        };
        return (completer, awaiter);
    }

    fn complete(&mut self, data: OpaqueData) -> Result<(), ()>{
        if let Some(channel) = self.sender.take() {
            channel.send(data).map_err(drop)
        } else {
            Err(())
        }
    }
}

pub struct AwaitExtern {
    receiver: Receiver<OpaqueData>
}

impl AwaitExtern {
    //TODO platform error
    pub async fn await_extern(self) -> Result<OpaqueData, PlatformError> {
        self.receiver.await.map_err(Into::into)
    }
}


#[repr(C)]
pub struct PlatformError {

}

impl From<Canceled> for PlatformError {
    fn from(_: Canceled) -> Self {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use static_assertions::assert_obj_safe;

    use super::*;

    #[test]
    fn test_static_assertions() {
        assert_obj_safe!(RustCompletesAsyncCall);
    }
}