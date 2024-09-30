use libc::{c_char, c_int};
use std::ffi::CString;

#[link(name = "epd")]
extern {
    fn DisplayBmp(image_path: *const c_char, x: c_int, y: c_int, width: c_int, height: c_int) -> libc::c_uchar;
    fn DisplayBmpFromMemory(image_data: *const u8, x: c_int, y: c_int, width: c_int, height: c_int) -> libc::c_uchar;
    fn Close();
    fn Clear();
}

pub fn clear() {
    unsafe {
        Clear();
    }
}

pub fn display_bmp_from_memory(image_data: &[u8], x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        let image_data_ptr = image_data.as_ptr();
        DisplayBmpFromMemory(image_data_ptr as *const c_char, x, y, width, height);
    }
}

pub fn display_bmp(image_path: &str, x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        let c_path = CString::new(image_path).expect("String conversion failed");
        DisplayBmp(c_path.as_ptr(), x, y, width, height);
    }
}
