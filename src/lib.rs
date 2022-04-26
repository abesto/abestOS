#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::testlib::test_runner)]
#![reexport_test_harness_main = "test_harness_main"]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod printlib;
pub mod qemu;
pub mod serial;
pub mod testlib;
pub mod vga_buffer;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    testlib::panic(info);
}

#[cfg(test)]
entry_point!(test_main);
#[cfg(test)]
fn test_main(_boot_info: &'static BootInfo) -> ! {
    test_harness_main();
    hlt_loop()
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    gdt::init();
    interrupts::init();
}
