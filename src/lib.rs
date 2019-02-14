#![cfg_attr(not(test), no_std)] 

pub mod vga_buffer;
pub mod serial;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}



// #[cfg(feature = "integration-test")]
// #[cfg(not(test))]
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     serial_println("Hello Host{}", "!");
    
//     // run tests
    
//     unsafe { exit_qemu(); }
//     loop {}
// }