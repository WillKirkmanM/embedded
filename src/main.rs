// We are not using the standard library
#![no_std]
// We are writing our own main function (entry point)
#![no_main]

use core::panic::PanicInfo;

/// The entry point for our program.
/// The `cortex_m_rt::entry` macro handles setting up the stack and jumping here.
#[cortex_m_rt::entry]
fn main() -> ! {
    // This is where our OS initialization and main loop will go.
    // For now, it just loops forever to prevent the microcontroller from halting.
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Loop forever on panic
    loop {}
}