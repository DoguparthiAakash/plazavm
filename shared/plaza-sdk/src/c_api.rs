use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn plaza_init() -> *mut c_void {
    std::ptr::null_mut()
}
