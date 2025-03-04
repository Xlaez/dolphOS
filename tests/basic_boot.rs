#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dolph_os::test_runner)]

use dolph_os::println;

#[test_case]
fn test_println() {
    println!("test_println output");
}

/*
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dolph_os::test_runner)]

use core::panic::PanicInfo;

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
    // test_main();

    // loop {}
// }

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
*/
