#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use libc::{strcpy, strlen};
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn blake3_hash_init(
    initid: *mut UDF_INIT,
    args: *mut UDF_ARGS,
    message: *mut ::std::os::raw::c_char,
) -> bool {
    if (*args).arg_count != 1 {
        strcpy(
            message,
            CString::new("blake3_hash must have one argument")
                .unwrap_or_default()
                .as_ptr(),
        );
        return true;
    }
    *((*args).arg_type) = Item_result_STRING_RESULT;
    (*initid).maybe_null = true;
    return false;
}

#[no_mangle]
pub unsafe extern "C" fn blake3_hash(
    _initid: *mut UDF_INIT,
    args: *mut UDF_ARGS,
    result: *mut ::std::os::raw::c_char,
    res_length: *mut ::std::os::raw::c_ulong,
    _null_value: *mut ::std::os::raw::c_uchar,
    _: *mut ::std::os::raw::c_uchar,
) -> *mut ::std::os::raw::c_char {
    let text = *((*args).args);

    strcpy(result, text);
    *res_length = strlen(text) as u64;

    return result;
}
