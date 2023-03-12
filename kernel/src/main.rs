#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic. Has to be defined because of no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
