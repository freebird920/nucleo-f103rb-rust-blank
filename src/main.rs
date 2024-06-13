#![no_std]
#![no_main]
#![allow(unused_parens)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::peripherals::rcc;
use cortex_m_rt::{entry, exception};
use panic_halt as _;
use peripherals::{
    afio::{EXTIx_Px, AFIO},
    exti::EXTI,
    gpio::{GPIOx_BASE, GPIO},
    i2c::{I2C, I2C_BASE, PCF8574_LCD},
    nvic::{NVIC, NVIC_BASE},
    tim_gp,
};
use rtt_target::{rprintln, rtt_init_print};
use utils::delay::delay_sys_clk_ms;

mod peripherals;
mod utils;

// const
const PCF8574_ADDRESS: u8 = 0b100111;
const RCC_BASE: u32 = 0x4002_1000;
const EXTI_BASE: u32 = 0x4001_0400;
const AFIO_BASE: u32 = 0x4001_0000;

// static
static COUNT: AtomicU32 = AtomicU32::new(0);
static REFRESH_LCD: AtomicBool = AtomicBool::new(true);

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let rcc = rcc::RCC::new(RCC_BASE);
    let gpio_a = GPIO::new(GPIOx_BASE::A);
    let gpio_b = GPIO::new(GPIOx_BASE::B);
    let gpio_c = GPIO::new(GPIOx_BASE::C);
    let afio = AFIO::new(AFIO_BASE);
    let exti = EXTI::new(EXTI_BASE);
    let nvic: NVIC = NVIC::new(NVIC_BASE);
    unsafe {
        // Enable GPIOA, GPIOB and GPIOC clocks
        rcc.CR_HSION();
        // rcc.set_sys_clock_32MHz();
        rcc.set_sys_clock_64MHz();
        let cr_val = rcc.read_cr();
        rprintln!("CR: {}", cr_val);
        let cfgr_val = rcc.read_cfgr();
        rprintln!("CFGR: {}", cfgr_val);

        rcc.APB1ENR_TIMxEN(rcc::TIMxEN::TIM2EN, true);

        rcc.ABP1ENR_I2C2EN(true);

        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPAEN, true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPBEN, true);
        rcc.APB2ENR_IOPx_EN(rcc::IOPxEN::IOPCEN, true);

        rcc.ABP2ENR_AFIOEN(true);

        gpio_a.crl_port_config(5, 0b0001); // Configure GPIOA pin 5 as output push-pull
        gpio_b.crh_port_config(10, 0b1001); // Configure GPIOC pin 10 as output open-drain
        gpio_b.crh_port_config(11, 0b1010); // Configure GPIOC pin 11 as output open-drain

        gpio_c.crh_port_config(13, 0b0100); // PC13 is input mode
        rprintln!("GPIO initialized");

        // interrupt configuration

        afio.exti_cr_x(EXTIx_Px::PC, 13); // Configure EXTI13 to use PC13
        exti.imr_set(13, true); // Enable interrupt mask register for EXTI13
        exti.ftsr_set(13, true); // Enable rising edge trigger on EXTI13
                                 // exti.rstr_set(13, true);
        nvic.enable_interrupt(40); // Enable EXTI15_10 interrupt

        let i2c2 = I2C::new(I2C_BASE::BASE_I2C2);
        i2c2.init();
        rprintln!("I2C2 initialized");
        let lcd = PCF8574_LCD::new(i2c2, PCF8574_ADDRESS);
        lcd.lcd_initialize();
        rprintln!("LCD initialized");

        let pllrdy = rcc.read_cr_pllrdy();
        rprintln!("PLL ready: {}", pllrdy);
        let mut loop_count = 0;
        // lcd.print("Hell");
        loop {
            let count = COUNT.load(Ordering::Relaxed).into();
            if REFRESH_LCD.load(Ordering::Relaxed) {
                REFRESH_LCD.store(false, Ordering::Relaxed);
                lcd.set_cursor(0, 0);
                lcd.clear();
                delay_sys_clk_ms(100);
                lcd.set_cursor(0, 0);
                delay_sys_clk_ms(100);
                lcd.print("Hello");
                lcd.set_cursor(0, 6);
                lcd.print_number(loop_count);
                delay_sys_clk_ms(100);
                lcd.set_cursor(1, 2);
                delay_sys_clk_ms(100);
                lcd.print_number(count);
                delay_sys_clk_ms(100);
            } else {
                loop_count += 1;
                lcd.set_cursor(0, 6);
                lcd.print_number(loop_count);
                delay_sys_clk_ms(1000);
                // REFRESH_LCD.store(true, Ordering::Relaxed);
            }
            // lcd.clear();
            // delay_sys_clk_ms(100);
            // lcd.set_cursor(0, 0);
            // delay_sys_clk_ms(100);
            // lcd.print("Hello kor");
            // delay_sys_clk_ms(100);
            // lcd.set_cursor(1, 2);
            // delay_sys_clk_ms(100);
            // lcd.print_number(count);
            // delay_sys_clk_ms(100);
        }
    }
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    rprintln!("Unhandled exception (IRQn = {})", irqn);
    let exti = EXTI::new(EXTI_BASE);
    let pr_13 = exti.pr_read(13);
    if (irqn == 40) && pr_13 {
        rprintln!("EXTI13 interrupt");
        COUNT.fetch_add(1, Ordering::Relaxed);
        REFRESH_LCD.store(true, Ordering::Relaxed);
        exti.pr_clear(13);
    }
}
