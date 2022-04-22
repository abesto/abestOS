use crate::println;
use spin::Lazy;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt
});

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    err_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT {}\n{:#?}", err_code, stack_frame);
}

pub fn init() {
    IDT.load();
}
