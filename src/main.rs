#![no_std]
#![no_main]
#![allow(unused_parens)]
// #![deny(unsafe_code)]

use cortex_m_rt::entry;
use panic_halt as _;

const RCC_BASE: u32 = 0x4002_1000;
const GPIOA_BASE: u32 = 0x4001_0800;
const GPIOC_BASE: u32 = 0x4001_1000;

#[entry]
fn main() -> ! {
    const RCC_APB2ENR: *mut u32 = (RCC_BASE + 0x18) as *mut u32;
    const GPIOA_CRL: *mut u32 = (GPIOA_BASE + 0x00) as *mut u32;
    const GPIOA_BSRR: *mut u32 = (GPIOA_BASE + 0x10) as *mut u32;
    const GPIOC_IDR: *mut u32 = (GPIOC_BASE + 0x08) as *mut u32;
    const GPIOC_CRH: *mut u32 = (GPIOC_BASE + 0x04) as *mut u32;

    unsafe {
        // Enable GPIOA and GPIOC clocks
        RCC_APB2ENR.write_volatile(RCC_APB2ENR.read_volatile() | (1 << 2) | (1 << 4));

        // Set GPIOA pin 5 to output push-pull
        let mut crl_val = GPIOA_CRL.read_volatile();
        crl_val &= !(0b1111 << 20); // Clear CNF5[1:0] and MODE5[1:0]
        crl_val |= (0b0001 << 20); // Set MODE5[1:0] to 01 (Output mode, max speed 10 MHz)
        GPIOA_CRL.write_volatile(crl_val);

        // Set GPIOC pin 13 to input floating (reset state)
        let mut crh_val = GPIOC_CRH.read_volatile();
        crh_val &= !(0b1111 << 20); // Clear CNF13[1:0] and MODE13[1:0]
        crh_val |= (0b0100 << 20); // Set CNF13[1:0] to 01 (Floating input)
        GPIOC_CRH.write_volatile(crh_val);
    }

    loop {
        unsafe {
            // Check if button is pressed (PC13 is low)
            if GPIOC_IDR.read_volatile() & (1 << 13) == 0 {
                // Set PA5 low (LED off)
                GPIOA_BSRR.write_volatile(1 << (5 + 16));
            } else {
                // Set PA5 high (LED on)
                GPIOA_BSRR.write_volatile(1 << 5);
            }
        }
    }
}
