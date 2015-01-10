extern crate libc;

use self::libc::{ c_void, c_char };

#[link(name = "dt", kind = "static")]
extern "C" {
    pub fn create_vm() -> *const c_void;
    pub fn eval(vm: *const c_void, javascript: *const c_char) -> *const c_char;
    pub fn destroy_vm(vm: *const c_void);
}
