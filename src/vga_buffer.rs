use spin::{Lazy, Mutex};
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    #[must_use]
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        Self::new(Color::White, Color::Black)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl Default for ScreenChar {
    fn default() -> Self {
        Self {
            ascii_character: b' ',
            color_code: ColorCode::default(),
        }
    }
}

impl core::ops::Deref for ScreenChar {
    type Target = Self;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl core::ops::DerefMut for ScreenChar {
    fn deref_mut(&mut self) -> &mut Self {
        self
    }
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    #[must_use]
    fn width(&self) -> usize {
        BUFFER_WIDTH
    }

    fn height(&self) -> usize {
        BUFFER_HEIGHT
    }
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    #[must_use]
    pub fn new() -> Self {
        let raw = 0xb8000 as *mut Buffer;
        Self {
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { raw.as_mut().unwrap() },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= self.buffer.width() {
                    self.new_line();
                }
                self.buffer.chars[self.buffer.height() - 1][self.column_position].update(|c| {
                    c.ascii_character = byte;
                    c.color_code = self.color_code;
                });
                self.column_position += 1;
            }
        }
    }

    pub fn new_line(&mut self) {
        for col in 0..self.buffer.width() {
            for row in 1..self.buffer.height() {
                self.buffer.chars[row - 1][col].write(self.buffer.chars[row][col].read());
            }
            self.buffer.chars[self.buffer.height() - 1][col].write(ScreenChar::default());
        }
        self.column_position = 0;
    }

    pub fn set_color_code(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }

    pub fn reset_color(&mut self) {
        self.color_code = ColorCode::default();
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub static WRITER: Lazy<Mutex<Writer>> = Lazy::new(|| Mutex::new(Writer::new()));

macro_rules! print {
    ($($arg:tt)*) => {
        write!(*$crate::vga_buffer::WRITER.lock(), $($arg)*).unwrap()
    };
}
pub(crate) use print;

macro_rules! println {
    ($($arg:tt)*) => {
        writeln!(*$crate::vga_buffer::WRITER.lock(), $($arg)*).unwrap()
    };
}
pub(crate) use println;

pub fn set_color_code(foreground: Color, background: Color) {
    WRITER.lock().set_color_code(foreground, background);
}

pub fn reset_color() {
    WRITER.lock().reset_color();
}
