use std::{marker::PhantomPinned, mem::ManuallyDrop};



/// Opaque type, similar but not quite the same as c_void
///
/// Also this type is guaranteed to be sendable.
#[repr(C)]
pub struct OpaqueSendable {
    /// See [Nomicon](https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs)
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, PhantomPinned)>,
}

unsafe impl Send for OpaqueSendable {}

#[repr(C)]
pub struct OpaqueData {
    ptr: Option<Box<ManuallyDrop<OpaqueSendable>>>,
    drop_fn: extern "C" fn(&mut ManuallyDrop<OpaqueSendable>),
}

impl OpaqueData {
    pub fn move_out(data: &mut OpaqueData) -> OpaqueData {
        OpaqueData {
            ptr: data.ptr.take(),
            drop_fn: data.drop_fn,
        }
    }
    fn drop_inner_value(&mut self) {
        if let Some(mut not_null) = self.ptr.take() {
            (self.drop_fn)(&mut not_null);
        }
    }
}

impl Drop for OpaqueData {
    fn drop(&mut self) {
        self.drop_inner_value();
    }
}

/// Drops OpaqueData in place.
///
/// After calling it OpaqueData is still valid, but it's data
/// point is set to a null pointer. This avoids certain
/// "copy" pitfalls around FFI. As the inner pointer can
/// always be null this shouldn't introduce any unexpected
/// null-pointers.
#[no_mangle]
pub extern "C" fn drop_opaque_data(data: &mut OpaqueData) {
    data.drop_inner_value();
}

pub fn drop_copy_opaque_data(_: &mut ManuallyDrop<OpaqueSendable>) {}


#[cfg(test)]
mod tests {
    use static_assertions::{assert_impl_all, assert_not_impl_any};

    use super::*;

    #[test]
    fn marker_bounds_are_correct() {
        assert_impl_all!(OpaqueData: Send, Unpin);
        assert_not_impl_any!(OpaqueData: Sync);

        assert_impl_all!(OpaqueSendable: Send);
        assert_not_impl_any!(OpaqueSendable: Sync, Unpin);
    }
}