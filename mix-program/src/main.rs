extern crate core;
use core::ffi::c_int;

#[link(name = "multiply", kind = "static")]
extern "C" {
    fn multiply(a: c_int, b: c_int) -> c_int;
}

fn main() {
    println!("[Rust] Hello from Rust! 🦀");

    let result = unsafe { multiply(1, 2) };
    println!("1 * 2 = {}", result);
}
