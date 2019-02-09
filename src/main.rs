#![cfg_attr(not(test), no_std)] // Disable the standard library
#![cfg_attr(test, allow(unused_imports))] // Allow unused imports when testing
#![cfg_attr(not(test), no_main)] // We don't have access to the C runtime library so main cannot be called 

use core::panic::PanicInfo;
mod vga_buffer;

#[cfg(not(test))] // Tests run with std library
#[no_mangle] // Ensure the linker finds _start instead of a mangled name
// Entry point on Linux
pub extern "C" fn _start() -> ! { //extern C defines the C calling convention
    println!("Welcome to Ros");
    loop {}
}

// We need to define our own panic handler, Rust's panic handler is provided by std
#[cfg(not(test))]  // Tests run with std library so don't compile this then 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
