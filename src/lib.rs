#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use libc::strcpy;
use std::ffi::{CStr, CString};

fn write_result(result_ptr: *mut ::std::os::raw::c_char, message: &[u8]) -> u64 {
    unsafe {
        let message_cstring = CString::from_vec_unchecked(message.to_vec());
        let message_cstring_raw = message_cstring.into_raw();
        strcpy(result_ptr, message_cstring_raw);
        drop(CString::from_raw(message_cstring_raw));
    }
    return message.len() as u64;
}

#[no_mangle]
pub unsafe extern "C" fn blake3_hash_init(
    initid: *mut UDF_INIT,
    args: *mut UDF_ARGS,
    message: *mut ::std::os::raw::c_char,
) -> bool {
    if (*args).arg_count != 1 {
        write_result(message, b"blake3_hash must have one argument");
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
    is_null: *mut ::std::os::raw::c_uchar,
    _error: *mut ::std::os::raw::c_uchar,
) -> *mut ::std::os::raw::c_char {
    let text = *((*args).args);

    if text.is_null() {
        *is_null = 1;
        return result;
    }

    let text_cstr = CStr::from_ptr(text);
    let text_hash = blake3::hash(text_cstr.to_bytes());
    let text_hash_hex = text_hash.to_hex();

    *res_length = write_result(result, text_hash_hex.as_bytes());

    return result;
}
