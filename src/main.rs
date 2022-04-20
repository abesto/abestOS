#![no_std]
#![no_main]
#![feature(once_cell)]

mod vga_buffer;

use bootloader::{entry_point, BootInfo};
use core::fmt::Write;
use core::panic::PanicInfo;

use vga_buffer::{println, reset_color, set_color_code, Color};

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_color_code(Color::White, Color::Red);
    println!("{}", info);
    loop {}
}

entry_point!(kernel_main);

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
