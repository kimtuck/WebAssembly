use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

fn my_string_safe(i: *mut c_char) -> String {
    unsafe {
        CStr::from_ptr(i).to_string_lossy().into_owned()
    }
}

#[no_mangle]
pub fn fix_story(xin: *mut c_char) -> *mut c_char {
    let data = my_string_safe(xin);
    let out = data.replace("one", "twice");

    CString::new(out)
    .unwrap()
    .into_raw()
}

fn main() {
    // Deliberately blank.
}