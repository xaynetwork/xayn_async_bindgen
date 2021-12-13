#![doc(hidden)]
pub struct Api2;

#[doc = r" Initializes the dart api."]
#[doc = r""]
#[doc = r" Is safe to be called multiple times and form multiple"]
#[doc = r" thread."]
#[doc = r""]
#[doc = r" # Safety"]
#[doc = r""]
#[doc = r" Must be called with a pointer produced by dart using"]
#[doc = r" `NativeApi.initializeApiDLData`."]
#[no_mangle]
pub unsafe extern "C" fn async_bindgen_dart_init_api__api2(
    init_data: *mut ::std::ffi::c_void,
) -> u8 {
    let res = unsafe { ::dart_api_dl::initialize_dart_api_dl(init_data) };
    res.is_ok() as u8
}

#[doc = r" Wrapper for initiating the call to an async function."]
#[no_mangle]
pub extern "C" fn async_bindgen_dart_c__api2__get_the_byte(
    async_bindgen_dart_port_id: ::async_bindgen::dart::DartPortId,
    async_bindgen_dart_completer_id: i64,
) -> u8 {
    match ::async_bindgen::dart::PreparedCompleter::new(
        async_bindgen_dart_port_id,
        async_bindgen_dart_completer_id,
    ) {
        Ok(completer) => {
            completer.spawn(Api2::get_the_byte());
            1
        }
        Err(_) => 0,
    }
}

#[doc = r#" Extern "C"  wrapper delegating to `PreparedCompleter::extract_result()`."#]
#[doc = r""]
#[doc = r" # Safety"]
#[doc = r""]
#[doc = r" See the language specific version of `PreparedCompleter::extract_result()`."]
#[no_mangle]
pub unsafe extern "C" fn async_bindgen_dart_r__api2__get_the_byte(handle: i64) -> u8 {
    unsafe { ::async_bindgen::dart::PreparedCompleter::extract_result(handle) }
}
