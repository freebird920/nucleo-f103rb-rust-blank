#![no_std]
#![no_main]
#![allow(unused_parens)]
use cortex_m_rt::entry;
use panic_halt as _;
use peripherals::{
    gpio::{GPIOx_BASE, GPIO},
    tim_gp,
};
use rtt_target::{rprintln, rtt_init_print};
use utils::delay::{delay_sys_clk_ms, delay_sys_clk_us};

mod external;
mod peripherals;
mod utils;
// use crate::peripherals::gpio;
use crate::peripherals::rcc;

const RCC_BASE: u32 = 0x4002_1000;

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
        let cfgr_val = rcc.read_cfgr();
        rprintln!("CFGR: {}", cfgr_val);
        rcc.APB1ENR_TIMxEN(rcc::TIMxEN::TIM2EN, true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPAEN, true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPCEN, true);

        gpio_a.crl_port_config(5, 0b0001); // Configure GPIOA pin 5 as output push-pull
        gpio_c.crh_port_config(13, 0b0100); // PC13 is input mode
        rprintln!("GPIO initialized")
    }
    loop {
        if gpio_c.idr_read(13) == 0 {
            rprintln!("Button pressed");
            delay_sys_clk_ms(1000);
        } else {
            rprintln!("Echo ... on");
            gpio_a.bsrr_write(5);
            delay_sys_clk_ms(1000);
            rprintln!("Echo ... off");
            gpio_a.bsrr_reset(5);
            delay_sys_clk_ms(1000);
        }
    }
}
