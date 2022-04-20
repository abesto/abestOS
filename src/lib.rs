#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testlib::test_runner)]
#![reexport_test_harness_main = "test_harness_main"]

use bootloader::{entry_point, BootInfo};

pub mod printlib;
pub mod qemu;
pub mod serial;
pub mod testlib;
pub mod vga_buffer;

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    testlib::panic(info);
}

#[cfg(test)]
entry_point!(test_main);
fn test_main(_boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    test_harness_main();
    loop {}
}