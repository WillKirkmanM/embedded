use cortex_m::interrupt::{self, Mutex};
use core::cell::RefCell;

// A global variable to count system ticks.
// The Mutex from cortex_m ensures safe access from both the main loop and the ISR.
static G_TICKS: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

// This function will be our Interrupt Service Routine.
// The `#[interrupt]` attribute registers it in the interrupt vector table.
#[cortex_m_rt::interrupt]
fn TIM2() {
    // Acknowledge the interrupt to prevent it from firing again immediately.
    // This involves writing to a specific register in the timer peripheral.
    // unsafe { (*TIMER_STATUS_REGISTER).write_volatile(0); }
    
    // Safely update the global tick counter.
    interrupt::free(|cs| {
        G_TICKS.borrow(cs).replace_with(|&mut old| old + 1);
    });
}

// In main, we would configure the TIM2 timer to fire an interrupt every 1ms.
#[cortex_m_rt::entry]
fn main() -> ! {
    // 1. Configure the TIM2 peripheral to trigger an interrupt every 1ms.
    // 2. Enable the TIM2 interrupt in the Nested Vectored Interrupt Controller (NVIC).
    
    // Now the TIM2 ISR will run automatically in the background.
    loop {
        // We can read the tick count safely here.
        let current_ticks = interrupt::free(|cs| {
            *G_TICKS.borrow(cs).borrow()
        });
    }
}