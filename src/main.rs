#![no_std] // dont link rust std lib
#![no_main] // disable rust-level entry point

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // keep name of the function as is
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
