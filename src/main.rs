#![no_std]
#![no_main]
#![allow(unused_parens)]

use core::sync::atomic::{AtomicU32, Ordering};
use core_peripherals::nvic::Nvic;
use cortex_m_rt::{entry, exception};
use panic_halt as _;
use peripherals::{adc::Adc, afio::Afio, exti::Exti, gpio::Gpio};

mod core_peripherals;
mod peripherals;
mod utils;

use crate::core_peripherals::scb::Scb;
use crate::peripherals::rcc::Rcc;
use crate::peripherals::stk::Stk;

#[allow(unused)]
use crate::peripherals::tim_gp::TimGp;
use core::cell::RefCell;

use cortex_m::{
    asm::delay,
    delay,
    interrupt::{self, Mutex},
};

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
    let _ = Gpio::new(2)
        .and_then(|gpio_c| {
            gpio_c.gpio_clock_enable()?;
            gpio_c.cr_pin_config(13, 0b0100)?; // PC13 -> Input mode with pull-up / pull-down -- USER BUTTON
            gpio_c.cr_pin_config(0, 0b0000)?;
            gpio_c.cr_pin_config(1, 0b0000)?;
            rprintln!("cr 0: {}", gpio_c.cr_read(0)?);
            Ok(gpio_c)
        })
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("Gpio C Init")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));


    // SET AFIO
    let afio = Afio::new();
    afio.afio_clock_enable();
    rprintln!("ABP2ENR: {}", rcc.abp2enr_read());
    let _ = afio
        .exti_cr(2, 13)
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("AFIO Port-C Pin-13 Set")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));
    let afio_exticr_read = afio.afio_exticr_read(4);
    rprintln!("AFIO_EXTICR4: {}", afio_exticr_read.unwrap_or(0));
    // EXTI 세팅
    let exti = Exti::new();
    let _ = exti.imr_set(13, true);
    exti.ftsr_set(13, true);
    // Nvic 세팅
    let nvic = Nvic::new(40)
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("Nvic Set")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));
    nvic.as_ref()
        .map(|nvic| {
            nvic.iser_set(40, true);
        })
        .ok();

    // ADC 세팅
    let adc = Adc::new(1)
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("ADC Set")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));

    adc.as_ref().ok().map(|adc| {
        adc.cr2_adon(true); // ADC ON
        adc.cr2_cont(true); // 연속변환모드
        adc.cr2_cal(); // 보정
        let _ = adc.smpr_smp(10,0b101);
        let _ = adc.smpr_smp(11,0b101);
        adc.sqr3_sq(1, 10);
        adc.sqr3_sq(2, 10);
        adc.cr2_swstart(true);

    });

    loop {
        // rprintln!("Loop");
        gpio_a.as_ref().ok().map(|gpio| {
            gpio.bsrr_write(5); // Set PA5 (LD2)
        });
        // let read_value = gpio_c.as_ref().map(|gpio| gpio.idr_read(13)).unwrap_or(0);
        // rprintln!("Read Value: {}", read_value);
        // delay(9_000*1000);
    }
}
#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    rprintln!("Unhandled exception (IRQn = {})", irqn);
    match irqn {
        40 => {
            rprintln!("EXTI15_10");
            let exti = Exti::new();
            let pr = exti.pr_read(13);
            rprintln!("PR: {}", pr);
            exti.pr_clear(13);
        }
        _ => (),
    }
    // Exti::new().pr_clear(13);
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
