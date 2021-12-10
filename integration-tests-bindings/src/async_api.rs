pub struct AsyncApi ; #[no_mangle] pub unsafe extern "C" fn
async_bindgen_dart_init_api__async_api(init_data : * mut :: std :: ffi ::
                                       c_void) -> u8
{ :: dart_api_dl :: initialize_dart_api_dl(init_data).is_ok() as u8 }
#[no_mangle] pub extern "C" fn
async_bindgen_dart_c__async_api__add(x : u8, y : u8,
                                     async_bindgen_dart_port_id : ::
                                     async_bindgen :: dart :: DartPortId,
                                     async_bindgen_dart_completer_id : i64) ->
u8
{
    match :: async_bindgen :: dart :: PreparedCompleter ::
    new(async_bindgen_dart_port_id, async_bindgen_dart_completer_id)
    {
        Ok(completer) => { completer.spawn(AsyncApi :: add(x, y)) ; 1 } Err(_)
        => 0
    }
} #[no_mangle] pub unsafe extern "C" fn
async_bindgen_dart_r__async_api__add(handle : i64) -> u8
{
    unsafe
    {
        :: async_bindgen :: dart :: PreparedCompleter ::
        extract_result(handle)
    }
} #[no_mangle] pub extern "C" fn
async_bindgen_dart_c__async_api__sub(x : u8, y : u8,
                                     async_bindgen_dart_port_id : ::
                                     async_bindgen :: dart :: DartPortId,
                                     async_bindgen_dart_completer_id : i64) ->
u8
{
    match :: async_bindgen :: dart :: PreparedCompleter ::
    new(async_bindgen_dart_port_id, async_bindgen_dart_completer_id)
    {
        Ok(completer) => { completer.spawn(AsyncApi :: sub(x, y)) ; 1 } Err(_)
        => 0
    }
} #[no_mangle] pub unsafe extern "C" fn
async_bindgen_dart_r__async_api__sub(handle : i64) -> u8
{
    unsafe
    {
        :: async_bindgen :: dart :: PreparedCompleter ::
        extract_result(handle)
    }
}