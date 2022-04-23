#![no_std]
#![no_main]

use abest_os::{
    print, println,
    qemu::{exit_qemu, QemuExitCode},
};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    abest_os::init();
    should_panic();
    println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    abest_os::hlt_loop()
}

fn should_panic() {
    print!("double_fault::should_panic...\t");
    unsafe {
        let mut _x = *(0xdeadbeef as *mut u32);
        _x = 32;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    abest_os::hlt_loop()
}
