use crate::qemu::{exit_qemu, QemuExitCode};
use crate::{print, println};

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::vga_buffer::set_color_code(
        crate::vga_buffer::Color::White,
        crate::vga_buffer::Color::Red,
    );
    println!("{}", info);
    crate::qemu::exit_qemu(crate::qemu::QemuExitCode::Failed);
    crate::hlt_loop()
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
