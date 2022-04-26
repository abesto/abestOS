#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(abest_os::testlib::test_runner)]
#![reexport_test_harness_main = "test_harness_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};

#[cfg(not(test))]
use abest_os::println;
#[cfg(not(test))]
use abest_os::vga_buffer::{reset_color, set_color_code, Color};

#[cfg(not(test))]
entry_point!(kernel_main);
#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    abest_os::init();

    set_color_code(Color::Cyan, Color::DarkGray);
    println!("Hello world! {}", 42);
    println!();

    set_color_code(Color::Yellow, Color::DarkGray);
    println!("Hello world! {:?}", Color::Yellow);

    reset_color();
    println!("No colors now!");

    x86_64::instructions::interrupts::int3();

    println!("Still here");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { abest_os::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    abest_os::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0

    use abest_os::memory::BootInfoFrameAllocator;
    use alloc::{boxed::*, rc::*, vec, vec::*};
    use x86_64::VirtAddr;
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    abest_os::hlt_loop()
}

#[cfg(test)]
entry_point!(test_main);
#[cfg(test)]
fn test_main(_boot_info: &'static BootInfo) -> ! {
    abest_os::init();
    #[cfg(test)]
    test_harness_main();
    abest_os::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    abest_os::testlib::panic(info);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use x86_64::instructions::interrupts::without_interrupts;

    abest_os::vga_buffer::set_color_code(
        abest_os::vga_buffer::Color::White,
        abest_os::vga_buffer::Color::Red,
    );
    println!("{}", info);
    without_interrupts(abest_os::hlt_loop)
}
