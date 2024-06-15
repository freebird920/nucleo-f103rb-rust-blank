#![no_std]
#![no_main]
#![allow(unused_parens)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use cortex_m_rt::{entry, exception};
// use cortex_m::interrupt::{Mutex};
use panic_halt as _;
use peripherals::{
    afio::{EXTIx_Px, AFIO}, exti::exti, gpio::{GpioXBase, Gpio}, i2c::{I2C, I2C_BASE, PCF8574_LCD}, nvic::{NVIC, NVIC_BASE}, rcc::{rcc, IOPxEN, TIMxEN}
};
use rtt_target::{rprintln, rtt_init_print};
use utils::delay::delay_sys_clk_ms;

mod peripherals;
mod utils;

// const
const PCF8574_ADDRESS: u8 = 0b100111;
const EXTI_BASE: u32 = 0x4001_0400;
const AFIO_BASE: u32 = 0x4001_0000;

// static
static COUNT: AtomicU32 = AtomicU32::new(0);
static REFRESH_LCD: AtomicBool = AtomicBool::new(true);

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let rcc = rcc::new();
    let gpio_a = Gpio::new(GpioXBase::A);
    let gpio_b = Gpio::new(GpioXBase::B);
    let gpio_c = Gpio::new(GpioXBase::C);
    let afio = AFIO::new(AFIO_BASE);
    let exti = exti::new(EXTI_BASE);
    let nvic: NVIC = NVIC::new(NVIC_BASE);
    // Enable GPIOA, GPIOB and GPIOC clocks
    rcc.cr_hsion();
    // rcc.set_sys_clock_32MHz();
    rcc.set_sys_clock_64MHz();

    let cr_val = rcc.read_cr();
    rprintln!("CR: {}", cr_val);
    let cfgr_val = rcc.read_cfgr();
    rprintln!("CFGR: {}", cfgr_val);

    rcc.cfgr_adcpre(11); // ADC prescaler 8

    rcc.APB2ENR_ADC1EN(true);


    rcc.APB1ENR_TIMxEN(TIMxEN::TIM2EN, true);
    rcc.ABP1ENR_I2C2EN(true);
    rcc.APB2ENR_IOPx_EN(IOPxEN::IOPAEN, true);
    rcc.APB2ENR_IOPx_EN(IOPxEN::IOPBEN, true);
    rcc.APB2ENR_IOPx_EN(IOPxEN::IOPCEN, true);

    rcc.ABP2ENR_AFIOEN(true);

    gpio_a.crl_port_config(5, 0b0001); // Configure GPIOA pin 5 as output push-pull
    gpio_b.crh_port_config(10, 0b1001); // Configure GPIOC pin 10 as output open-drain
    gpio_b.crh_port_config(11, 0b1010); // Configure GPIOC pin 11 as output open-drain

    gpio_c.crh_port_config(13, 0b0100); // PC13 is input mode

    let sysclk = get_sys_clock();
    rprintln!("System clock: {} Hz", sysclk);
   
    // PC0 ADC12_IN10  PC1 ADC12_IN11

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
            cortex_m::asm::delay(sysclk);
            // delay_sys_clk_ms(1000);
            // REFRESH_LCD.store(true, Ordering::Relaxed);
        }
    }
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    rprintln!("Unhandled exception (IRQn = {})", irqn);
    let exti = exti::new(EXTI_BASE);
    let pr_13 = exti.pr_read(13);
    if (irqn == 40) && pr_13 {
        rprintln!("EXTI13 interrupt");
        COUNT.fetch_add(1, Ordering::Relaxed);
        REFRESH_LCD.store(true, Ordering::Relaxed);
        exti.pr_clear(13);
    }
}


fn get_sys_clock() -> u32 {
    // RCC 레지스터 하드코딩된 값
    let rcc = rcc::new();
    let rcc_cr = rcc.read_cr(); // 예시 값, 실제 값으로 교체 필요
    let rcc_cfgr = rcc.read_cfgr(); // 예시 값, 실제 값으로 교체 필요

    // HSI 클럭 속도
    let hsi_clk = 8_000_000; // 8 MHz
    // HSE 클럭 속도 (기본값)
    let hse_clk = 8_000_000; // 8 MHz

    // SWS 비트 확인 (시스템 클럭 소스)
    let sws = (rcc_cfgr >> 2) & 0b11;

    let sysclk = match sws {
        0b00 => {
            // HSI 사용
            hsi_clk
        }
        0b01 => {
            // HSE 사용
            hse_clk
        }
        0b10 => {
            // PLL 사용
            // PLL 소스 확인 (PLLSRC 비트)
            let pllsrc = (rcc_cfgr >> 16) & 0b1;
            let pll_clk_in = if pllsrc == 0 {
                // HSI/2
                hsi_clk / 2
            } else {
                // HSE
                if (rcc_cfgr >> 17) & 0b1 == 1 {
                    hse_clk / 2
                } else {
                    hse_clk
                }
            };

            // PLL 곱셈 계수 확인 (PLLMUL 비트)
            let pllmul = ((rcc_cfgr >> 18) & 0b1111) + 2;
            pll_clk_in * pllmul
        }
        _ => {
            // 예약된 값 (사용되지 않음)
            0
        }
    };

    // AHB 프리스케일러 확인 (HPRE 비트)
    let hpre = (rcc_cfgr >> 4) & 0b1111;
    let ahb_prescaler = match hpre {
        0b0000 => 1,
        0b1000 => 2,
        0b1001 => 4,
        0b1010 => 8,
        0b1011 => 16,
        0b1100 => 64,
        0b1101 => 128,
        0b1110 => 256,
        0b1111 => 512,
        _ => 1,
    };

    sysclk / ahb_prescaler
}