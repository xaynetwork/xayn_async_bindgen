use std::ffi::c_void;

use dart_api_dl::initialize_dart_api_dl;

#[rustfmt::skip]
mod async_api;

//yes the struct is generated by us
// as well as a `pub use ::async_bindgen_bindings::<name>::*;`
// we use that as we don't really know where in the file tree this
// file is.
#[async_bindgen::api(
    // spawner = AsyncStdSpawner, will affect the extern "C" constructor
    //....
)]
impl AsyncApi {
    pub async fn add(x: u8, y: u8) -> u8 {
        x + y
    }
}

#[no_mangle]
pub unsafe extern "C" fn init_dart_api_dl(native_ptr: *mut c_void) -> bool {
    initialize_dart_api_dl(native_ptr).is_ok()
}
