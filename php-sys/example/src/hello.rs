extern crate php_sys;

use php_sys::*;
use std::mem;
use std::ptr;
use std::ffi::CString;

const PHP_MODULE_NAME : &str = "hello";
const PHP_HELLO_VERSION : &str = "0.1.0";

fn strpprintf(max_len: usize, format: &str) -> *mut zend_string {
    let c_format = CString::new(format).unwrap();
    unsafe {
        let strg = zend_strpprintf(max_len, c_format.as_ptr());
        strg
    }
}

extern "C" fn zif_confirm_hello_compiled(_execute_data: *mut zend_execute_data, return_value: *mut zval) { // void
    let strg = strpprintf(0, "Congratulations! You have successfully modified ext/hello/config.m4. Module hello is now compiled into PHP.");
    unsafe {
        (*return_value).value.str = strg; // RETURN_STR
        (*return_value).u1.type_info = 5126;
    }
}

#[no_mangle]
pub extern "C" fn get_module() -> *const zend_module_entry {
    let mut hello_functions: Vec<zend_function_entry> = vec![
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

    let mut hello_module_entry :zend_module_entry = Default::default();
    hello_module_entry.size = mem::size_of::<zend_module_entry>() as std::os::raw::c_ushort;
    hello_module_entry.name = CString::new(PHP_MODULE_NAME).unwrap().into_raw();
    hello_module_entry.version = CString::new(PHP_HELLO_VERSION).unwrap().into_raw();
    hello_module_entry.functions = hello_functions.as_mut_ptr() as *const _zend_function_entry;
    mem::forget(hello_functions);
    mem::forget(hello_module_entry);
    &hello_module_entry
}