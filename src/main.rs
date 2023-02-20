use std::os::raw::{c_char, c_int};
use std::str::FromStr;
use std::{
    any::type_name,
    ffi::{CStr, CString},
    mem::{align_of, size_of},
    time::SystemTime,
};
extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn get_string(s: *mut *const c_char);
}

fn main() {
    let rust_str = "Hello from C!";
    let start = get_now_as_micros();
    let null_terminated = CString::new(rust_str).unwrap();
    let format = "%s\n".as_ptr() as *const c_char;
    unsafe {
        printf(format, null_terminated.as_ptr());
    }
    println!("{}", get_now_as_micros() - start);
    let start = get_now_as_micros();
    println!("{}", rust_str);
    println!("{}", get_now_as_micros() - start);
}

fn get_now_as_micros() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

fn print_size<T>() {
    println!(
        "type: {} ,size: {}, align: {}",
        type_name::<T>(),
        size_of::<T>(),
        align_of::<T>()
    );
}
