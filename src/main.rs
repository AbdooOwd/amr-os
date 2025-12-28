#![no_std]
#![no_main]

mod vga_buffer;
mod types;

use core::panic::PanicInfo;
use core::fmt::Write;
use crate::vga_buffer::{Color, ColorCode, Writer};

const AMROS_HEADER_MSG: &str = "AmrOS - v0.1.0";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new(ColorCode::new(Color::White, Color::Black));

    writer.print_string_hcenter(AMROS_HEADER_MSG, 0);

    /* to test scrolling
    for i in 0..35 {
        write!(writer, "This is line {i}").unwrap();
    }
    */

    #[allow(clippy::empty_loop)]
    loop {} // Clippy gets angry cuz we're "wasting" CPU cycles

    // for now we loop. in the future we should
    // implement something to power off or idk
    // something that the OS does when it "stops"
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
