#![no_std]  // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the Linker looks for a function
    // name `_start` by default.
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// Call this function on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
