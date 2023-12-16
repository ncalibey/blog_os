#![no_std]  // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

use core::fmt::Write;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

mod vga_buffer;

// This function is the entry point, since the Linker looks for a function
// name `_start` by default.
#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

/// Call this function on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
