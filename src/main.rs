#![no_std]  // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

use core::panic::PanicInfo;

#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the Linker looks for a function
    // name `_start` by default.
    loop {}
}

/// Call this function on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
