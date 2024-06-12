#![no_std]
#![no_main]
#![allow(unused_parens)]
use cortex_m_rt::entry;
use panic_halt as _;
use peripherals::{
    gpio::{GPIOx_BASE, GPIO},
    i2c::{I2C, I2C_BASE, PCF8574_LCD},
    tim_gp,
};
use rtt_target::{rprintln, rtt_init_print};
use utils::delay::{delay_sys_clk_10us, delay_sys_clk_ms};

mod external;
mod peripherals;
mod utils;
// use crate::peripherals::gpio;
use crate::peripherals::rcc;
const PCF8574_ADDRESS: u8 = 0b100111;
const RCC_BASE: u32 = 0x4002_1000;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let rcc = rcc::RCC::new(RCC_BASE);
    let tim2 = tim_gp::TIM_GP::new(tim_gp::TIM_GP_TYPE::TIM2);
    tim2.set_psc(32_000);
    let gpio_a = GPIO::new(GPIOx_BASE::A);
    let gpio_b = GPIO::new(GPIOx_BASE::B);
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
        rcc.ABP1ENR_I2C2EN(true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPAEN, true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPBEN, true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPCEN, true);
        gpio_a.crl_port_config(5, 0b0001); // Configure GPIOA pin 5 as output push-pull
                                           // gpio PB10 scl and PB11 sda
        gpio_b.crh_port_config(10, 0b1001); // Configure GPIOC pin 10 as output open-drain
        gpio_b.crh_port_config(11, 0b1010); // Configure GPIOC pin 11 as output open-drain
        gpio_c.crh_port_config(13, 0b0100); // PC13 is input mode
        rprintln!("GPIO initialized");

        let i2c2 = I2C::new(I2C_BASE::BASE_I2C2);
        i2c2.init();
        rprintln!("I2C2 initialized");
        let lcd = PCF8574_LCD::new(i2c2, PCF8574_ADDRESS);
        lcd.lcd_initialize();
        rprintln!("LCD initialized");


        // lcd.print("Hell");
        loop {
            if gpio_c.idr_read(13) == 0 {
                rprintln!("Button pressed");
                gpio_a.bsrr_write(5);
                lcd.display_off();
                delay_sys_clk_ms(10);
                lcd.lcd_initialize();
                delay_sys_clk_ms(1000);
                } else {
                lcd.clear();
                delay_sys_clk_ms(1000);
                lcd.set_cursor(0, 0);
                delay_sys_clk_ms(1000);
                lcd.print("Hel");
                delay_sys_clk_ms(10);
                lcd.set_cursor(1, 1);
                delay_sys_clk_ms(10);
                lcd.print("lo");
                delay_sys_clk_ms(1000);
                // rprintln!("Echo ... on");
                // gpio_a.bsrr_write(5);
                // rprintln!("Echo ... off");
                // gpio_a.bsrr_reset(5);
                // delay_sys_clk_ms(1000);
            }
        }
    }
}
