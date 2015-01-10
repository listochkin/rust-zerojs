#![allow(unstable)]

extern crate libc;

use std::ffi::{ CString, c_str_to_bytes};
use std::str::from_utf8;
use std::ops::Drop;
use self::libc::{ c_void };

mod dt;

pub struct Zjs {
    vm: *const c_void
}

impl Zjs {
    pub fn new() -> Zjs {
        let vm = unsafe { dt::create_vm() };
        Zjs { vm: vm }
    }
    pub fn run(&self, javascript: &str) -> String {
        let c_code = CString::from_slice(javascript.as_bytes());
        let c_code_ptr = c_code.as_ptr();
        let c_result = unsafe { dt::eval(self.vm, c_code_ptr) };
        let result_bytes = unsafe { c_str_to_bytes(&c_result) };

        // FIXME: CESU-8 to UTF-8 conversion
        // TODO: handle error states
        let result = from_utf8(result_bytes);
        assert_eq!(result.is_ok(), true);
        let result_option = result.ok();
        assert_eq!(result_option.is_some(), true);
        String::from_str(result_option.unwrap())
    }
}

impl Drop for Zjs {
    fn drop(&mut self) {
        unsafe {
            dt::destroy_vm(self.vm);
        }
    }
}

#[test]
fn test_public_api() {
    let code = "(function () { return '12'; })()";
    let zjs = Zjs::new();
    let result: String = zjs.run(code);
    assert_eq!(result.as_slice(), "12");
}

pub fn dt_hello_world() {
    unsafe {
        let vm = dt::create_vm();
        let code = "(function () { return '12'; })()";
        let c_code = CString::from_slice(code.as_bytes());
        let c_result = dt::eval(vm, c_code.as_ptr());
        // FIXME: CESU-8 to UTF-8 conversion
        // TODO: handle error states
        let result = String::from_str(from_utf8(c_str_to_bytes(&c_result)).ok().unwrap());
        assert_eq!(result.as_slice(), "12");
        dt::destroy_vm(vm);
    }
}

#[test]
fn test_binding() {
    dt_hello_world();
}
