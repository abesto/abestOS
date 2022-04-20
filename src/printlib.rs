#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga_print!($($arg)*);
        $crate::serial_print!($($arg)*);
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::vga_println!($($arg)*);
        $crate::serial_println!($($arg)*);
    };
}
