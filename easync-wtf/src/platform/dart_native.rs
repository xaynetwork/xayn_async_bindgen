use std::mem::size_of;

use crate::opaque::OpaqueData;

use super::{RustAsyncCommand, RustCompletesAsyncCall};


///
///
/// This need to be wrapped in a `extern "C"` function for each specific
/// async command.
pub fn create_rust_to_dart_completer(
    port: DartPort,
    completer_id: CompleterId,
    command: impl RustAsyncCommand,
) {
    let completer: Box<dyn RustCompletesAsyncCall> = move |data| {
        let msg = Box::into_raw(Box::new(CompletionMsg {
            completer_id,
            data
        }));
        let msg = Box::into_raw(Box::new(Dart_CObject_as_native_ptr {
            r#type: Dart_CObject_kNativePointer,
            ptr: msg as usize,
            size: size_of::<CompletionMsg>(),
            callback: finalize_completion_msg
        }));
        unsafe {
            Dart_PostCObject(port, msg);
        }
    };
}

#[repr(C)]
struct Dart_CObject_as_native_ptr {
    r#type:
}

#[repr(C)]
pub struct CompletionMsg {
    completer_id: CompleterId,
    data: OpaqueData
}

extern "C" fn finalize_completion_msg(/*TODO*/) {
    todo!()
}

pub type CompleterId = u64;
pub type DartPort = u64;

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn test_assumtions() {
        assert_eq!(size_of::<usize>(), size_of::<*mut u8>());
    }
}