#![no_std]
#![no_main]

use abest_os::{
    print, println,
    qemu::{exit_qemu, QemuExitCode},
};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
