// A representation of a GPIO peripheral's memory-mapped registers.
// The addresses (e.g., 0x4002_0C00) are fictional but represent real hardware addresses
// found in a microcontroller's datasheet.
const GPIO_PORTC_BASE: usize = 0x4002_0C00;
const MODE_REGISTER_OFFSET: usize = 0x00;
const OUTPUT_DATA_REGISTER_OFFSET: usize = 0x14;

/// Represents a single GPIO pin
pub struct GpioPin {
    port_base: usize,
    pin_number: u8,
}

impl GpioPin {
    /// Creates a new GPIO pin and configures it as an output.
    pub fn new(port_base: usize, pin_number: u8) -> Self {
        let mode_reg = (port_base + MODE_REGISTER_OFFSET) as *mut u32;
        
        // This is an unsafe operation because we are writing directly to a memory address.
        // We trust that the address is correct and valid.
        unsafe {
            // Set the pin's mode to "General-purpose output mode" by writing to the register.
            // The `volatile` write prevents the compiler from optimizing this away.
            mode_reg.write_volatile(mode_reg.read_volatile() | (1 << (pin_number * 2)));
        }
        
        Self { port_base, pin_number }
    }

    /// Sets the pin's output level to high.
    pub fn set_high(&self) {
        let odr = (self.port_base + OUTPUT_DATA_REGISTER_OFFSET) as *mut u32;
        unsafe {
            odr.write_volatile(odr.read_volatile() | (1 << self.pin_number));
        }
    }

    /// Sets the pin's output level to low.
    pub fn set_low(&self) {
        let odr = (self.port_base + OUTPUT_DATA_REGISTER_OFFSET) as *mut u32;
        unsafe {
            odr.write_volatile(odr.read_volatile() & !(1 << self.pin_number));
        }
    }
}

// In our main function, we could now blink an LED on Pin 13 of Port C.
#[cortex_m_rt::entry]
fn main() -> ! {
    let led = GpioPin::new(GPIO_PORTC_BASE, 13);
    loop {
        led.set_high();
        // A simple delay loop (in a real OS, use a timer!)
        for _ in 0..1_000_000 {}
        led.set_low();
        for _ in 0..1_000_000 {}
    }
}