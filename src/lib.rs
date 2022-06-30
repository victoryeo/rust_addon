use nodejs_sys::{napi_create_string_utf8, napi_env, napi_set_named_property, napi_value};
use std::ffi::CString;
#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
// creating a C string
    let key = CString::new("hello").expect("CString::new failed");
// creating a memory location where the pointer to napi_value will be saved
    let mut local: napi_value = std::mem::zeroed();
// creating a C string
    let value = CString::new("world!").expect("CString::new failed");
// creating napi_value for the string
    napi_create_string_utf8(env, value.as_ptr(), 6, &mut local);
// setting the string on the exports object
    napi_set_named_property(env, exports, key.as_ptr(), local);
// returning the object 
    exports
}