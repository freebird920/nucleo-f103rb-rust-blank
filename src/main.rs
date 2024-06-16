#![no_std]
#![no_main]
#![allow(unused_parens)]

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::{entry, exception};
use panic_halt as _;
use peripherals::{afio::Afio, gpio::Gpio};

mod core_peripherals;
mod peripherals;
mod utils;

use crate::core_peripherals::scb::Scb;
use crate::peripherals::rcc::Rcc;
use crate::peripherals::stk::Stk;

#[allow(unused)]
use crate::peripherals::tim_gp::TimGp;
use core::cell::RefCell;

use cortex_m::interrupt::{self, Mutex};

use rtt_target::{rprintln, rtt_init_print};

static SYS_CLOCK: AtomicU32 = AtomicU32::new(0);
static PENDSV_COMMAND: Mutex<RefCell<Option<PendSVCommand>>> = Mutex::new(RefCell::new(None));

// PendSVCommand enum 정의
enum PendSVCommand {
    Log(&'static str),
}

// trigger_command 함수 정의
fn trigger_pend_sv(command: PendSVCommand) {
    interrupt::free(|cs| {
        *PENDSV_COMMAND.borrow(cs).borrow_mut() = Some(command);
    });
    // PendSV 인터럽트를 트리거
    Scb::new().icsr_pendsvset_write();
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let rcc = Rcc::new();

    rcc.set_sys_clock(0b1110); // 0b1110 -> 64Mhz
    SYS_CLOCK.store(rcc.get_sys_clock(), Ordering::Release); // SysClock 저장

    let stk = Stk::new();
    rprintln!(
        "stk_calib noref{} skew{} tenms{}",
        stk.calib_noref_read(),
        stk.calib_skew_read(),
        stk.calib_tenms_read()
    );

    rprintln!("System clock: {} Hz", SYS_CLOCK.load(Ordering::Acquire));
    // let _ = match TimGp::new(2) {
    //     Ok(tim2) => {
    //         tim2.tim_gp_clock_enable();
    //         trigger_pend_sv(PendSVCommand::Log("TimGp Set"));
    //     }
    //     Err(e) => {
    //         trigger_pend_sv(PendSVCommand::Log(e.trim()));
    //     }
    // };

    // GPIO A 초기화
    let gpio_a = Gpio::new(0)
        .and_then(|gpio_a| {
            gpio_a.gpio_clock_enable()?;
            gpio_a.cr_pin_config(5, 0b0001)?; // CNFy + MODEx
            Ok(gpio_a)
        })
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("Gpio A Init")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));

    // GPIO C 초기화
    let gpio_c = Gpio::new(2)
        .and_then(|gpio_c| {
            gpio_c.gpio_clock_enable()?;
            gpio_c.cr_pin_config(13, 0b1010)?; // CNFy + MODEx
            Ok(gpio_c)
        })
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("Gpio C Init")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));

    // AFIO 세팅
    let afio = Afio::new();
    afio.afio_clock_enable();
    let _ = afio
        .exti_cr(2, 13)
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("AFIO Port 2 Pin 13 Set")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));

    loop {
        // rprintln!("Loop");
        gpio_a
            .as_ref()
            .map(|gpio| {
                gpio.bsrr_write(5); // Set PA5 (LD2)
            })
            .ok();
    }
}

#[exception]
fn PendSV() {
    interrupt::free(|cs| {
        if let Some(command) = PENDSV_COMMAND.borrow(cs).take() {
            match command {
                PendSVCommand::Log(message) => rprintln!("{}", message),
            }
        }
    });
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    rprintln!("Unhandled exception (IRQn = {})", irqn);
}
