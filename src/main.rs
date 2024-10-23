#![no_std] // dont link rust std lib
#![no_main] // disable rust-level entry point
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[no_mangle] // keep name of the function as is
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    // panic!("Some panic message!");
    loop {}
}

#[test_case]
fn trivial_assert() {
    print!("trivial assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}
