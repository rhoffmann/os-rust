#![no_std] // dont link rust std lib
#![no_main] // disable rust-level entry point
#![feature(custom_test_frameworks)] // adapt test framework bc default depends on std lib which is not linked
#![reexport_test_harness_main = "test_main"]
#![test_runner(os_rust::test_runner)]

use core::panic::PanicInfo;
use os_rust::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_rust::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
