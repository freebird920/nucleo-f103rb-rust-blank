#![no_std]
#![no_main]
#![allow(unused_parens)]

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::{entry, exception};
use cortex_m::asm::delay;
use panic_halt as _;

use cortex_m::interrupt::{self, Mutex};
use cortex_m::peripheral::{SCB};
use core::cell::RefCell;
use crate::peripherals::rcc::Rcc;
use crate::peripherals::stk::Stk;
use crate::peripherals::tim_gp::TimGp;


use rtt_target::{rprintln, rtt_init_print};

mod peripherals;
mod utils;
// const PCF8574_ADDRESS: u8 = 0b100111;
// static COUNT: AtomicU32 = AtomicU32::new(0);
// static REFRESH_LCD: AtomicBool = AtomicBool::new(true);
static SYS_CLOCK: AtomicU32 = AtomicU32::new(0);
static PENDSV_MESSAGE: Mutex<RefCell<Option<&'static str>>> = Mutex::new(RefCell::new(None));

// test_interrupt 함수 정의
fn test_interrupt(num: u8) -> Result<(), &'static str> {
    match num {
        1 => Ok(()),
        _ => {
            trigger_error_interrupt("Error occurred in test_interrupt() function.");
            Err("Invalid number")},
    }
}




#[entry]
fn main() -> ! {
    rtt_init_print!();
    let rcc = Rcc::new();

    rcc.set_sys_clock(0b1110); // 0b1110 -> 64Mhz
    SYS_CLOCK.store(rcc.get_sys_clock(), Ordering::Release); // SysClock 저장
    
    let stk = Stk::new();
    rprintln!("stk_calib noref{} skew{} tenms{}",stk.calib_noref_read(),stk.calib_skew_read(),stk.calib_tenms_read());

    rprintln!("System clock: {} Hz", SYS_CLOCK.load(Ordering::Acquire));

    test_interrupt(4);
    // TIM2 
    let tim2 = TimGp::new(2);
    tim2.tim_gp_clock_enable();


    loop {
        rprintln!("Hello, world!");
        delay(SYS_CLOCK.load(Ordering::Acquire));
    }
}

// #[exception]
// unsafe fn DefaultHandler(irqn: i16) {

fn trigger_error_interrupt(message: &'static str) {
    interrupt::free(|cs| {
        *PENDSV_MESSAGE.borrow(cs).borrow_mut() = Some(message);
    });
    // PendSV 인터럽트를 트리거
    unsafe {
        (*SCB::PTR).icsr.write(1 << 28);
    }
}


// #[exception]
// unsafe fn DefaultHandler(irqn: i16) {
//     trigger_error_interrupt("Error occurred in DefaultHandler() function.");
// }

#[exception]
fn PendSV() {
    interrupt::free(|cs| {
        if let Some(message) = PENDSV_MESSAGE.borrow(cs).take() {
            rprintln!("{}", message);
        }
    });
}
