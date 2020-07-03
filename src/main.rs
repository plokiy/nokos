#![no_std] // force not to link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // force not to mangle the name of this function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

mod vga_buffer;

