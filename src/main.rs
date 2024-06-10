#![no_std]
#![no_main]
#![allow(unused_parens)]
// #![deny(unsafe_code)]

use cortex_m_rt::entry;
use panic_halt as _;

mod utils;
mod peripherals;
mod external;

use crate::peripherals::rcc;

const RCC_BASE: u32 = 0x4002_1000;
// const PCF8574_LCD_ADDRESS: u8 = 0x27;
// const RCC_APB2ENR:  *mut u32 = (RCC_BASE + 0x18) as *mut u32;
// const RCC_APB1ENR:  *mut u32 = (RCC_BASE + 0x1C) as *mut u32;
// const RCC_CR:       *mut u32 = (RCC_BASE + 0x00) as *mut u32;

// unsafe fn rcc_hsion (){
//     let mut rcc_cr_val: u32 = RCC_CR.read_volatile();
//     rcc_cr_val |= (1 << 0);// HSION
//     RCC_CR.write_volatile(rcc_cr_val);
// }

#[entry]
fn main() -> ! {
    let rcc = rcc::RCC::new(RCC_BASE);
    // let tim2 = peripherals::tim::TIM2::new();
    unsafe {
        // Enable GPIOA, GPIOB and GPIOC clocks
        rcc.CR_HSION();
        rcc.set_sys_clock_32MHz();

        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPAEN, true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPCEN, true);

        // let mut apb2enr_val: u32 = RCC_APB2ENR.read_volatile();
        // apb2enr_val |= (1 << 2) |  // IOPAEN 
        //                (1 << 3) |  // IOPBEN
        //                (1 << 4);   // IOPCEN
        // RCC_APB2ENR.write_volatile(apb2enr_val);

        // let mut apb1enr_val: u32 = RCC_APB1ENR.read_volatile();
        // apb1enr_val |= (1 << 21);   // I2C2EN
        // RCC_APB1ENR.write_volatile(apb1enr_val); 

        // peripherals::i2c::init();
        peripherals::gpio::init();
        // tim2.init(32000 - 1, 1); // 프리스케일러와 ARR 값을 설정
        // Initialize the LCD
        // external::pcf8574_lcd::lcd_init(PCF8574_LCD_ADDRESS);
    }

    loop {
        unsafe {
            // Check if button is pressed (PC13 is low)
            if peripherals::gpio::read_button() == 0 {
                peripherals::gpio::led_off();
            } else {
                peripherals::gpio::led_on();
                utils::delay::delay_ms(1000);
                // tim2.delay_ms(1000);
                peripherals::gpio::led_off();
                utils::delay::delay_ms(1000);
                // tim2.delay_ms(1000);
            }
        }
    }
}