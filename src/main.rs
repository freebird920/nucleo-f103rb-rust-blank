#![no_std]
#![no_main]
#![allow(unused_parens)]
// #![deny(unsafe_code)]
use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use peripherals::{gpio::{GPIOx_BASE, GPIO}, tim_gp};
use rtt_target::{rprintln, rtt_init_print};

mod external;
mod peripherals;
mod utils;
// use crate::peripherals::gpio;
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

static mut TIM2_INSTANCE : Option<tim_gp::TIM_GP> = None;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let rcc = rcc::RCC::new(RCC_BASE);
    let tim2 = tim_gp::TIM_GP::new(tim_gp::TIM_GP_TYPE::TIM2);
    tim2.set_psc(32_000);
    let gpio_a = GPIO::new(GPIOx_BASE::A);
    let gpio_c = GPIO::new(GPIOx_BASE::C);
    unsafe {
        // Enable GPIOA, GPIOB and GPIOC clocks
        rcc.CR_HSION();
        rcc.set_sys_clock_32MHz();
        let cr_val = rcc.read_cr();
        rprintln!("CR: {}", cr_val);
        let cfgr_val =     rcc.read_cfgr();
        rprintln!("CFGR: {}", cfgr_val);
        rcc.APB1ENR_TIMxEN(rcc::TIMxEN::TIM2EN, true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPAEN, true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPCEN, true);

        gpio_a.port_config(5, 0b00, 0b11);
        gpio_c.port_config(13, 0b01, 0b00); // PC13 is input mode
        peripherals::gpio::init();
        rprintln!("GPIO initialized")
    }
    let mut count = 0;
    loop {
        count = count +1 ;
        unsafe {
            // count = count + 1;
            // rprintln!("Loop count: {}", count);
            // Check if button is pressed (PC13 is low)
            if peripherals::gpio::read_button() == 0 {
                peripherals::gpio::led_off();
            } else {
                peripherals::gpio::led_on();
            }

        }
        // rprintln!("Loop count: {}", count);
    }
}
