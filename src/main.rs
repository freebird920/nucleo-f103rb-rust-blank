#![no_std]
#![no_main]
#![allow(unused_parens)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use cortex_m::asm::delay;
use cortex_m_rt::{entry, exception};
use panic_halt as _;

use crate::peripherals::rcc::rcc;
use rtt_target::{rprintln, rtt_init_print};

mod peripherals;
mod utils;

// const
// const PCF8574_ADDRESS: u8 = 0b100111;
// const EXTI_BASE: u32 = 0x4001_0400;
// const AFIO_BASE: u32 = 0x4001_0000;

// static
// static COUNT: AtomicU32 = AtomicU32::new(0);
// static REFRESH_LCD: AtomicBool = AtomicBool::new(true);
static sys_clock: AtomicU32 = AtomicU32::new(0);
#[entry]
fn main() -> ! {
    rtt_init_print!();
    let rcc = rcc::new();
    sys_clock.store(rcc.get_sys_clock(), Ordering::Release);
    rprintln!("System clock: {} Hz", sys_clock.load(Ordering::Acquire));
    loop {
        rprintln!("Hello, world!");
        delay(sys_clock.load(Ordering::Acquire));
    }
}

// #[exception]
// unsafe fn DefaultHandler(irqn: i16) {
//     rprintln!("Unhandled exception (IRQn = {})", irqn);
//     let exti = exti::new(EXTI_BASE);
//     let pr_13 = exti.pr_read(13);
//     if (irqn == 40) && pr_13 {
//         rprintln!("EXTI13 interrupt");
//         COUNT.fetch_add(1, Ordering::Relaxed);
//         REFRESH_LCD.store(true, Ordering::Relaxed);
//         exti.pr_clear(13);
//     }
// }

// // fn get_sys_clock() -> u32 {
//     // RCC 레지스터 하드코딩된 값
//     let rcc = rcc::new();
//     let rcc_cfgr = rcc.read_cfgr(); // 예시 값, 실제 값으로 교체 필요

//     // HSI 클럭 속도
//     let hsi_clk = 8_000_000; // 8 MHz
//     // HSE 클럭 속도 (기본값)
//     let hse_clk = 8_000_000; // 8 MHz

//     // SWS 비트 확인 (시스템 클럭 소스)
//     let sws = (rcc_cfgr >> 2) & 0b11;

//     let sysclk = match sws {
//         0b00 => {
//             // HSI 사용
//             hsi_clk
//         }
//         0b01 => {
//             // HSE 사용
//             hse_clk
//         }
//         0b10 => {
//             // PLL 사용
//             // PLL 소스 확인 (PLLSRC 비트)
//             let pllsrc = (rcc_cfgr >> 16) & 0b1;
//             let pll_clk_in = if pllsrc == 0 {
//                 // HSI/2
//                 hsi_clk / 2
//             } else {
//                 // HSE
//                 if (rcc_cfgr >> 17) & 0b1 == 1 {
//                     hse_clk / 2
//                 } else {
//                     hse_clk
//                 }
//             };

//             // PLL 곱셈 계수 확인 (PLLMUL 비트)
//             let pllmul = ((rcc_cfgr >> 18) & 0b1111) + 2;
//             pll_clk_in * pllmul
//         }
//         _ => {
//             // 예약된 값 (사용되지 않음)
//             0
//         }
//     };

//     // AHB 프리스케일러 확인 (HPRE 비트)
//     let hpre = (rcc_cfgr >> 4) & 0b1111;
//     let ahb_prescaler = match hpre {
//         0b0000 => 1,
//         0b1000 => 2,
//         0b1001 => 4,
//         0b1010 => 8,
//         0b1011 => 16,
//         0b1100 => 64,
//         0b1101 => 128,
//         0b1110 => 256,
//         0b1111 => 512,
//         _ => 1,
//     };

//     sysclk / ahb_prescaler
// }
