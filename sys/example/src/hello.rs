extern crate sys;

use sys::*;
use std::mem;
use std::ptr;
use std::ffi::CString;

const PHP_MODULE_NAME : &str = "hello";
const PHP_HELLO_VERSION : &str = "0.1.0";

extern "C" fn zif_confirm_hello_compiled(_execute_data: *mut zend_execute_data, _return_value: *mut zval) { // void
    println!("Congratulations! Module hello is now compiled into PHP.");
}

#[no_mangle]
pub extern "C" fn get_module() -> *const zend_module_entry {
    let hello_functions = vec![
        // PHP_FE
        zend_function_entry {
            fname: CString::new("confirm_hello_compiled").unwrap().into_raw(),
            handler:  Some(zif_confirm_hello_compiled),
            arg_info: ptr::null_mut(),
            num_args: 0,
            flags: 0,
        },
        zend_function_entry {
            fname: ptr::null_mut(),
            handler:  None,
            arg_info: ptr::null_mut(),
            num_args: 0,
            flags: 0,
        },
    ];

    let mut hello_module_entry : Box<zend_module_entry> = Default::default();
    hello_module_entry.zend_api = 20170718;
    hello_module_entry.build_id = CString::new("API20170718,NTS,debug").unwrap().into_raw();
    hello_module_entry.size = mem::size_of::<zend_module_entry>() as std::os::raw::c_ushort;
    hello_module_entry.name = CString::new(PHP_MODULE_NAME).unwrap().into_raw();
    hello_module_entry.version = CString::new(PHP_HELLO_VERSION).unwrap().into_raw();
    hello_module_entry.functions = hello_functions.as_ptr();

    mem::forget(hello_functions);
    Box::into_raw(hello_module_entry) as *const zend_module_entry
}