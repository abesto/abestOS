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
    abest_os::init();
    test_harness_main();
    abest_os::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    abest_os::testlib::panic(info);
}

#[test_case]
fn test_sanity() {
    println!("test_println output");
}

#[test_case]
fn test_breakpoint() {
    x86_64::instructions::interrupts::int3();
}
