#![no_std]  // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]

use blog_os::println;
use core::panic::PanicInfo;

// This function is the entry point, since the Linker looks for a function
// name `_start` by default.
#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// Call this function on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
