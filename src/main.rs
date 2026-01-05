#![no_std]
#![no_main]

mod vga_buffer;
mod types;

use core::fmt::Write;
use core::panic::PanicInfo;
use crate::vga_buffer::WRITER;

const AMROS_HEADER_MSG: &str = "AmrOS - v0.1.0";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    WRITER.lock().print_string_hcenter(AMROS_HEADER_MSG, 0);
    // reset the cursor position because of the header message
    WRITER.lock().cursor_set_position(0, 2);

    for i in 0..100 {
        writeln!(WRITER.lock(), "line {i}").unwrap();
    }

    loop {}

    // for now we loop. in the future we should
    // implement something to power off or idk
    // something that the OS does when it "stops"
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
