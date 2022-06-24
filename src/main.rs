#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rulos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rulos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rulos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
