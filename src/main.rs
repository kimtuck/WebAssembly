use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

fn my_string_safe(i: *mut c_char) -> String {
    unsafe {
        CStr::from_ptr(i).to_string_lossy().into_owned()
    }
}

#[no_mangle]
pub fn replace(text: *mut c_char, from: *mut c_char, to: *mut c_char) -> *mut c_char {
    let text = my_string_safe(text);
    let from = my_string_safe(from);
    let to = my_string_safe(to);
    let out = text.replace(&from, &to);

    CString::new(out)
    .unwrap()
    .into_raw()
}

fn main() {
    // Deliberately blank.
}