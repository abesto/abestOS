use spin::Lazy;
use x86_64::registers::segmentation::SegmentSelector;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        // TODO proper stack allocation once we have some memory management set up
        //      (ideally including a guard page)
        let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
        stack_start + STACK_SIZE
    };
    tss
});

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

struct GDTInfo {
    gdt: GlobalDescriptorTable,
    selectors: Selectors,
}

static GDTINFO: Lazy<GDTInfo> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    GDTInfo {
        gdt,
        selectors: Selectors {
            code_selector,
            tss_selector,
        },
    }
});

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDTINFO.gdt.load();
    unsafe {
        CS::set_reg(GDTINFO.selectors.code_selector);
        load_tss(GDTINFO.selectors.tss_selector);
    }
}
