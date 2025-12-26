#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
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
