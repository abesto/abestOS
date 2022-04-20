#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(abest_os::testlib::test_runner)]
#![reexport_test_harness_main = "test_harness_main"]

use core::panic::PanicInfo;

use abest_os::println;
use bootloader::{entry_point, BootInfo};

entry_point!(test_main);
fn test_main(_boot_info: &'static BootInfo) -> ! {
    test_harness_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    abest_os::testlib::panic(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
