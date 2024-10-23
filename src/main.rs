#![no_std] // dont link rust std lib
#![no_main] // disable rust-level entry point

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // keep name of the function as is
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // panic!("Some panic message!");
    loop {}
}
