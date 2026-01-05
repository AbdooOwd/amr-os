// TODO: give this file another name

use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::types::HAlignment;


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(
        Writer::new(ColorCode::new(Color::White, Color::Black))
    );
}

// I doubt we'll ever use any other address/buffer.
// Thus, let's just make this the default for the Writer.
const VGA_BUF_ADDR: usize = 0xB8000;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black   = 0x0,
    Blue    = 0x1,
    Green   = 0x2,
    Cyan    = 0x3,
    Red     = 0x4,
    Magenta = 0x5,
    Brown   = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
    White = 0xF
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // TODO: what's this for?
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/**
* the VGA buffer is 80 wide and 25 rows high.
* For each character, 2 bytes are use:
* One for the character, and the other for the color code
* (4 bits for foreground and 4 other bits for background)
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // Copy how C stores structs?
struct CharCell {
    character: u8,
    color: ColorCode
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<CharCell>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    /** Keeps track of the horizontal position of the *imaginary* cursor */
    col_position: usize, 
    row_position: usize,
    /** Color used by everything printed by this writer */
    color_scheme: ColorCode, 
    buffer: &'static mut Buffer
}


impl Writer {
    pub fn new(color_scheme: ColorCode) -> Writer {
        Writer {
            col_position: 0,
            row_position: 0,
            color_scheme,
            buffer: unsafe { &mut *(VGA_BUF_ADDR as *mut Buffer) } }
    }

    // TODO: Are these `cursor_set_` functions really necessary?
    pub fn cursor_set_column(&mut self, col: usize) {
        self.col_position = col;
    }

    pub fn cursor_set_row(&mut self, row: usize) {
        self.row_position = row;
    }

    pub fn cursor_set_position(&mut self, col: usize, row: usize) {
        self.cursor_set_column(col);
        self.cursor_set_row(row);
    }

    pub fn clear_line(&mut self, row: usize) {
        for i in 0..BUFFER_WIDTH {
            self.write_byte(b' ', i, row);
        }
    }

    pub fn clear_screen(&mut self) {
        for i in 0..BUFFER_HEIGHT {
            self.clear_line(i);
        }
    }

    pub fn scroll(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let byte = self.buffer.chars[row][col].read().character;
                self.write_byte(byte, col, row - 1);
            }
        }
        self.clear_line(BUFFER_HEIGHT - 1);
        self.col_position = 0;
        self.row_position = BUFFER_HEIGHT - 1;
    }

    /**
    * Prints character's byte at a specific position
    */
    pub fn write_byte(&mut self, byte: u8, col: usize, row: usize) {
        if row >= BUFFER_HEIGHT {
            panic!();
        }

        match byte {
            b'\n' => {
                // no fancying. we returnin to the zero!
                self.row_position = row + 1;
                self.col_position = 0;
            },
            b'\r' => {
                // the carriage-return char's job is returning to
                // the beginning of the line. so here we are
                self.col_position = 0;
            },
            byte => {
                self.buffer.chars[row][col].write(
                    CharCell {
                        character: byte,
                        color: self.color_scheme
                    }
                );

                self.col_position = col + 1;
                self.row_position = row;
            },

            /* handle other chars... */
        }

        // TODO: shouldn't be checked for all bytes.
        // There are bytes that just don't affect the cursor
        if self.col_position >= BUFFER_WIDTH && self.row_position < BUFFER_HEIGHT {
            self.col_position = 0;
            self.row_position += 1;
        }

        if self.row_position >= BUFFER_HEIGHT {
            self.scroll();
        }
    }

    /**
    * Just a wrapper to handle new lines
    *
    * TODO: Is this wrapper really necessary?
    */
    pub fn print_char_at(&mut self, byte: u8, col: usize, row: usize) {
        self.write_byte(byte, col, row);
    }

    /**
    * Prints character's byte at the Writer's cursor
    */
    pub fn print_char(&mut self, char: u8) {
        self.print_char_at(char, self.col_position, self.row_position);
    }

    pub fn print_string_at(&mut self, string: &str, col: usize, row: usize) {
        self.col_position = col;
        self.row_position = row;

        for byte in string.as_bytes() {
            match byte {
                0x20..=0x7E | b'\n' => { // printable ASCII characters
                    self.print_char(*byte);
                },
                _ => {
                    self.print_char(0xFE); // print a block
                }
            }
        }
    }

    /**
    *   Prints a string at vertical position `row` and horizontally aligned
    *   based on `halign` then align the text lines based on `text_align`.
    *
    *   Note: Text lines alignment currently only works with `halign` = center.
    *
    *   Note: Right text-lines alignment isn't supported yet
    *
    *   Note: That's a pretty pricy function in performance in my opinion
    *   for such a simple function
    */
    pub fn print_string_halign(&mut self, string: &str, halign: HAlignment, text_align: HAlignment, row: usize) {
        let string_lines = string.split('\n');
        let center_all_lines: bool = matches!(text_align, HAlignment::Center);

        // Explanation: if the text should be aligned to anything
        // other than center, then we use a boolean to tell
        // the coming for-loop that we don't need to center every line.

        let mut col_pos: usize = 0; // to be used in the for-loop
        for (i, line) in string_lines.enumerate() {
            // The condition below will always be true at least once.
            col_pos = match halign {
                HAlignment::Left => 0,
                HAlignment::Center => {
                    if center_all_lines || i < 1 {
                        (BUFFER_WIDTH / 2) - (line.len() / 2)
                    } else {
                        col_pos // don't change the value
                    }
                },
                HAlignment::Right => BUFFER_WIDTH - line.len()
            };
            self.print_string_at(line, col_pos, row + i);
        }
    }

    /**
    *   Just a wrapper for `print_string_halign` to print at the horizontal center
    *   at `row`.
    */
    pub fn print_string_hcenter(&mut self, string: &str, row: usize) {
        self.print_string_halign(string, HAlignment::Center, HAlignment::Center, row);
    }

    pub fn print_string(&mut self, string: &str) {
        self.print_string_at(string, self.col_position, self.row_position);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        Ok(())
    }
}
