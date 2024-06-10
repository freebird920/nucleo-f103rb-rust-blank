#![no_std]
#![no_main]
#![allow(unused_parens)]
// #![deny(unsafe_code)]

use cortex_m_rt::entry;
use panic_halt as _;

mod utils;
mod peripherals;
mod external;

const RCC_BASE: u32 = 0x4002_1000;
const PCF8574_LCD_ADDRESS: u8 = 0x27;

#[entry]
fn main() -> ! {
    const RCC_APB2ENR: *mut u32 = (RCC_BASE + 0x18) as *mut u32;
    const RCC_APB1ENR: *mut u32 = (RCC_BASE + 0x1C) as *mut u32;

    unsafe {
        // Enable GPIOA, GPIOB and GPIOC clocks
        let mut apb2enr_val: u32 = RCC_APB2ENR.read_volatile();
        apb2enr_val |= (1 << 2) |  // IOPAEN 
                       (1 << 3) |  // IOPBEN
                       (1 << 4);   // IOPCEN
        RCC_APB2ENR.write_volatile(apb2enr_val);

        let mut apb1enr_val: u32 = RCC_APB1ENR.read_volatile();
        apb1enr_val |= (1 << 21);   // I2C2EN
        RCC_APB1ENR.write_volatile(apb1enr_val); 

        peripherals::i2c::init();
        peripherals::gpio::init();

        // Initialize the LCD
        external::pcf8574_lcd::lcd_init(PCF8574_LCD_ADDRESS);
    }

    loop {
        unsafe {
            // Check if button is pressed (PC13 is low)
            if peripherals::gpio::read_button() == 0 {
                peripherals::gpio::led_on();
            } else {
                peripherals::gpio::led_on();
                utils::delay::delay_ms(1000);
                peripherals::gpio::led_off();
                utils::delay::delay_ms(1000);
            }
        }
    }
}