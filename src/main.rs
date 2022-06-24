#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rulos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rulos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rulos::init();
    // invoke a breakpoint exception
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rulos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
