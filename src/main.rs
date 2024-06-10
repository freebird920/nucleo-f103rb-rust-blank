#![no_std]
#![no_main]
#![allow(unused_parens)]
// #![deny(unsafe_code)]

use cortex_m_rt::entry;
use panic_halt as _;

mod peripherals;

const RCC_BASE  : u32 = 0x4002_1000;
// const GPIOA_BASE: u32 = 0x4001_0800;
// const GPIOB_BASE: u32 = 0x4001_0C00;
// const GPIOC_BASE: u32 = 0x4001_1000;
const I2C2_BASE : u32 = 0x4000_5800;
#[entry]
fn main() -> ! {
    const RCC_APB2ENR:  *mut u32    = (RCC_BASE + 0x18)         as *mut u32;
    const RCC_APB1ENR:  *mut u32    = (RCC_BASE + 0x1C)         as *mut u32;

    // const GPIOA_CRL:    *mut u32    = (GPIOA_BASE + 0x00)       as *mut u32;
    // const GPIOA_BSRR:   *mut u32    = (GPIOA_BASE + 0x10)       as *mut u32;

    // // const GPIOB_CRL:    *mut u32    = (GPIOB_BASE + 0x00)       as *mut u32;
    // const GPIOB_CRH:    *mut u32    = (GPIOB_BASE + 0x04)       as *mut u32;
    // const GPIOC_IDR:    *mut u32    = (GPIOC_BASE + 0x08)       as *mut u32;
    // const GPIOC_CRH:    *mut u32    = (GPIOC_BASE + 0x04)       as *mut u32;

    const I2C2_CR1:     *mut u32    = (I2C2_BASE + 0x00)        as *mut u32;
    const I2C2_CR2:     *mut u32    = (I2C2_BASE + 0x04)        as *mut u32;
    const I2C2_CCR:     *mut u32    = (I2C2_BASE + 0x1C)        as *mut u32;
    const I2C2_TRISE:   *mut u32    = (I2C2_BASE + 0x20)        as *mut u32;

    unsafe {
        // Enable GPIOA and GPIOC clocks
        let mut apb2enr_val: u32 = RCC_APB2ENR.read_volatile();
        apb2enr_val     |=  (1 << 2) |  // IOPAEN 
                            (1 << 3) |  // IOPBEN
                            (1 << 4);   // IOPCEN
        RCC_APB2ENR.write_volatile(apb2enr_val);

        let mut apb1enr_val : u32 = RCC_APB1ENR.read_volatile();
        apb1enr_val     |=  (1 << 21);   // I2C2EN
        RCC_APB1ENR.write_volatile(apb1enr_val); 
        
        // Set I2C2
        I2C2_CR1.write_volatile(I2C2_CR1.read_volatile() & !(1 << 0)); // Disable I2C2

        let mut i2c2_cr2_val = I2C2_CR2.read_volatile();
        i2c2_cr2_val &= !(0b111111 << 0); // Clear FREQ[5:0]
        i2c2_cr2_val |= (8 << 0); // Set FREQ[5:0] to 8MHz
        I2C2_CR2.write_volatile(i2c2_cr2_val);

        let mut i2c2_ccr_val = I2C2_CCR.read_volatile();
        i2c2_ccr_val &= !(0b1111_1111_1111 << 0); // Clear CCR[11:0]
        i2c2_ccr_val |= (40 << 0); // Set CCR[11:0] to 40 (Standard mode, 100 kHz)
        I2C2_CCR.write_volatile(i2c2_ccr_val);

        let mut i2c2_trise_val = I2C2_TRISE.read_volatile();
        i2c2_trise_val &= !(0b11111 << 0); // Clear TRISE[5:0]
        i2c2_trise_val |= (9 << 0); // Set TRISE[5:0] to 9
        I2C2_TRISE.write_volatile(i2c2_trise_val);

        I2C2_CR1.write_volatile(I2C2_CR1.read_volatile() | (1 << 0)); // Enable I2C2
        // End I2C2

        peripherals::gpio::init();

        // // Set GPIOA pin 5 to output push-pull
        // let mut gpioa_crl_val = GPIOA_CRL.read_volatile();
        // gpioa_crl_val &= !(0b1111 << 20); // Clear CNF5[1:0] and MODE5[1:0]
        // gpioa_crl_val |= (0b0001 << 20); // Set MODE5[1:0] to 01 (Output mode, max speed 10 MHz)
        // GPIOA_CRL.write_volatile(gpioa_crl_val);

        // // Set GPIOB pin 10 I2C2 SCL, pin 11 I2C2 SDA to alternate function open-drain
        // let mut gpiob_crh_val = GPIOB_CRH.read_volatile();
        // gpiob_crh_val &= !(0b1111 << 8); // Clear CNF10[1:0] and MODE10[1:0]
        // gpiob_crh_val &= !(0b1111 << 12); // Clear CNF11[1:0] and MODE11[1:0]
        // gpiob_crh_val |= (0b1011 << 8); // Set CNF10[1:0] to 10 (Alternate function output open-drain)
        // gpiob_crh_val |= (0b1011 << 12); // Set CNF11[1:0] to 10 (Alternate function output open-drain)
        // GPIOB_CRH.write_volatile(gpiob_crh_val);

        // // Set GPIOC pin 13 to input floating (reset state)
        // let mut gpioc_crh_val = GPIOC_CRH.read_volatile();
        // gpioc_crh_val &= !(0b1111 << 20); // Clear CNF13[1:0] and MODE13[1:0]
        // gpioc_crh_val |= (0b0100 << 20); // Set CNF13[1:0] to 01 (Floating input)
        // GPIOC_CRH.write_volatile(gpioc_crh_val);
    }

    loop {
        unsafe {
            // Check if button is pressed (PC13 is low)
            if peripherals::gpio::read_button() == 0 {
                peripherals::gpio::led_off();
            } else {
                peripherals::gpio::led_on();
            }
        }
    }
}