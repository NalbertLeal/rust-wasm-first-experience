// cargo  +nightly build --target wasm32-unknown-unknown --release
// wasm-gc target/wasm32-unknown-unknown/release/myproject.wasm target/wasm32-unknown-unknown/release/myproject-small.wasm

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub fn it_works() -> *mut c_char {
    let name = "nalbert";
    CString::new(name).unwrap().into_raw()
    // let to_return = CString::new(name).unwrap();
    // to_return.into_raw()
    // return String::from(a);
}

#[no_mangle]
pub fn it_works_size() -> usize {
    String::from("nalbert").len()
}