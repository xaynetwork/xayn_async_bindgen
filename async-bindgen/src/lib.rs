//TODO rename easync-dart-io-utils
// add easync-dart-js-utils
use std::future::Future;

use dart_api_dl::{cobject::OwnedCObject, ports::SendPort, DartRuntime};

pub use dart_api_dl::ports::DartPortId;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct CompleterId(i64);

impl From<CompleterId> for OwnedCObject {
    fn from(id: CompleterId) -> Self {
        OwnedCObject::int64(id.0)
    }
}

pub struct PreparedCompleter {
    send_port: Option<SendPort>,
    completer_id: CompleterId,
}

impl PreparedCompleter {
    pub fn new(port_id: i64, completer_id: i64) -> Result<Self, ()> {
        //TODO log err
        let rt = DartRuntime::instance().map_err(|_| ())?;
        let send_port = rt.send_port_from_raw(port_id).ok_or(())?;
        Ok(Self {
            send_port: Some(send_port),
            completer_id: CompleterId(completer_id),
        })
    }

    pub async fn bind_future<T>(mut self, future: impl Future<Output = T>)
    where
        T: Into<OwnedCObject>,
    {
        let output = future.await;
        let res = OwnedCObject::array(vec![
            Box::new(self.completer_id.into()),
            Box::new(output.into()),
        ]);
        self.send_result_if_not_already_done(res);
    }

    /// Sends the result
    ///
    /// Does nothing if the result was
    /// already send.
    fn send_result_if_not_already_done(&mut self, res: OwnedCObject) {
        if let Some(port) = self.send_port.take() {
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
        self.send_result_if_not_already_done(self.completer_id.into());
    }
}


/// Spawns the future.
///
// In the future this should allow different implementations (potentially
// with a cfg, for now only async-std as it's easier to use).
pub fn spawn(future: impl Future<Output = ()> + Send + 'static) {
    // noticeable more complex with tokio as:
    // - We need a handle to the runtime, but are not in the runtime
    //  - So we need to store the handle in some global slot and
    //    have a init runtime function.
    async_std::task::spawn(future);
}