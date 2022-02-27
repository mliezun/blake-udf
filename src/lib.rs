#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use blake3;
use std::ffi::CString;
use std::slice;

unsafe fn write_message(message_ptr: *mut ::std::os::raw::c_char, message: &[u8]) -> u64 {
    let c_message = match CString::new(message) {
        Ok(cs) => cs,
        Err(_) => return 0, // failed to convert to C string
    };
    let bytes = c_message.as_bytes_with_nul();

    let message_bytes = slice::from_raw_parts_mut(message_ptr as *mut u8, bytes.len() as usize);
    message_bytes[..bytes.len()].copy_from_slice(bytes);
    return bytes.len() as u64;
}

#[no_mangle]
pub unsafe extern "C" fn blake3_hash_init(
    initid: *mut UDF_INIT,
    args: *mut UDF_ARGS,
    message: *mut ::std::os::raw::c_char,
) -> bool {
    if (*args).arg_count != 1 {
        write_message(message, b"blake3_hash must have one argument");
        return true;
    }
    *(*args).arg_type = Item_result_STRING_RESULT;
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

    let hashed_text = blake3::hash(CString::from_raw(text).as_bytes());

    let res = hashed_text.as_bytes();

    *res_length = write_message(result, res);

    return result;
}
