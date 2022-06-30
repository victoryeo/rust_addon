use nodejs_sys::{
    napi_callback_info, napi_create_function, napi_env, napi_get_cb_info, napi_get_undefined,
    napi_get_value_string_utf8, napi_set_named_property, napi_value,
};

use std::ffi::CString;

pub unsafe extern "C" fn printme(env: napi_env, info: napi_callback_info) -> napi_value {
// creating a buffer of arguments
    let mut buffer: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();
    let mut argc = 1 as usize;
// getting arguments
    napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    let mut len = 0;
// getting length by passing null buffer
    napi_get_value_string_utf8(env, buffer[0], std::ptr::null_mut(), 0, &mut len);
    let size = len as usize;
// creating a buffer where string can be placed
    let mut ve: Vec<u8> = Vec::with_capacity(size + 1);
    let raw = ve.as_mut_ptr();
// telling rust not manage the vector
    std::mem::forget(ve);
    let mut cap = 0;
// getting the string value from napi_value
    let _s = napi_get_value_string_utf8(env, buffer[0], raw as *mut i8, size + 1, &mut cap);
    let s = String::from_raw_parts(raw, cap as usize, size);
// printing the string
    println!("{}", s);
// creating an undefined
    let mut und: napi_value = std::mem::zeroed();
    napi_get_undefined(env, &mut und);
// returning undefined
    und
}

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
    let p = CString::new("myFunc").expect("CString::new failed");
    let mut local: napi_value = std::mem::zeroed();
    napi_create_function(
        env,
        //pointer to function name
        p.as_ptr(),
        //length of function anme
        std::stringify!(printme).len(),
        //Rust function
        Some(printme),
        //context accessed by rust function
        std::ptr::null_mut(),
        //output napi value
        &mut local,
    );
    napi_set_named_property(env, exports, p.as_ptr(), local);
    exports
}