#![allow(unstable)]

use std::ffi::{ CString, c_str_to_bytes};
use std::str::from_utf8;

mod dt;

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
