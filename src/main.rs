#![no_std] // Disable the standard library
#![no_main] // We don't have access to the C runtime library so main cannot be called 

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// Entry point on Linux
#[no_mangle] // Ensure the linker finds _start instead of a mangled name
pub extern "C" fn _start() -> ! { //extern C defines the C calling convention
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// We need to define our own panic handler, Rust's panic handler is provided by std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Replace this later
    loop {}
}
