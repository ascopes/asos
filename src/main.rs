#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &b) in b"Hello, World!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = b;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
