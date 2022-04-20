#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(abest_os::testlib::test_runner)]
#![reexport_test_harness_main = "test_harness_main"]

use bootloader::{entry_point, BootInfo};

use abest_os::println;
use abest_os::vga_buffer::{reset_color, set_color_code, Color};

#[cfg(not(test))]
entry_point!(kernel_main);
#[cfg(test)]
entry_point!(test_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    set_color_code(Color::Cyan, Color::DarkGray);
    println!("Hello world! {}", 42);
    println!();

    set_color_code(Color::Yellow, Color::DarkGray);
    println!("Hello world! {:?}", Color::Yellow);

    reset_color();
    println!("No colors now!");

    None::<Option<u8>>.expect("Testing panic handler");

    loop {}
}

fn test_main(_boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    test_harness_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    abest_os::testlib::panic(info);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    abest_os::vga_buffer::set_color_code(
        abest_os::vga_buffer::Color::White,
        abest_os::vga_buffer::Color::Red,
    );
    println!("{}", info);
    loop {}
}
