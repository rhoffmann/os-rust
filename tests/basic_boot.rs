#![no_std] // dont link rust std lib
#![no_main] // disable rust-level entry point
#![feature(custom_test_frameworks)] // adapt test framework bc default depends on std lib which is not linked
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
