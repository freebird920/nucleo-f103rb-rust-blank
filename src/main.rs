#![no_std]
#![no_main]
#![allow(unused_parens)]

// core 라이브러리 사용
use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

// 모듈
mod core_peripherals;
mod external;
mod peripherals;
mod utils;

// 커스텀 라이브러리
use crate::core_peripherals::nvic::Nvic;
use crate::core_peripherals::scb::Scb;

use crate::peripherals::adc::Adc;
use crate::peripherals::afio::Afio;
use crate::peripherals::exti::Exti;
use crate::peripherals::gpio::Gpio;
use crate::peripherals::rcc::Rcc;
use crate::peripherals::stk::Stk;
use crate::peripherals::tim_gp::TimGp;

use crate::external::hd44780::Hd44780;
use crate::external::pcf8574::Pcf8574;
use cortex_m::{
    asm::delay,
    interrupt::{self, Mutex},
};
use cortex_m_rt::{entry, exception};
use external::hd44780;
use panic_halt as _;

use peripherals::i2c::I2c;
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

    // GPIO A 초기화
    let gpio_a = Gpio::new(0)
        .and_then(|gpio_a| {
            gpio_a.gpio_clock_enable()?;
            gpio_a.cr_pin_config(5, 0b0001)?; // CNFy + MODEx
            Ok(gpio_a)
        })
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("Gpio A Init")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));
    // GPIO B 초기화
    let gpio_b = Gpio::new(1)
        .and_then(|gpio_b| {
            gpio_b.gpio_clock_enable()?;
            gpio_b.cr_pin_config(10, 0b1001)?; // CNFy + MODEx
            gpio_b.cr_pin_config(11, 0b1010)?; // CNFy + MODEx

            Ok(gpio_b)
        })
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("Gpio B Init")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));

    // GPIO C 초기화
    let _ = Gpio::new(2)
        .and_then(|gpio_c| {
            gpio_c.gpio_clock_enable()?;
            gpio_c.cr_pin_config(13, 0b0100)?; // PC13 -> Input mode with pull-up / pull-down -- USER BUTTON
            gpio_c.cr_pin_config(0, 0b0000)?; // PC0 -> Analog mode (X axis of joystick)
            gpio_c.cr_pin_config(1, 0b0000)?; // PC1 -> Analog mode (Y axis of joystick)
            Ok(gpio_c)
        })
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("Gpio C Init")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));
    rprintln!("abp2enr: {}", rcc.apb2enr_read());
    // TIM2 세팅
    let tim_gp = TimGp::new(2)
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("TIM2 Set")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));

    // SET AFIO
    let afio = Afio::new();
    afio.afio_clock_enable();
    rprintln!("ABP2ENR: {}", rcc.apb2enr_read());
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
            nvic.iser_set(18, true);
            nvic.iser_set(40, true);
        })
        .ok();


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
        18 => {
            rprintln!("ADC1_2 interrupt");
            let adc = Adc::new(1);
            let dr_read = adc.as_ref().map(|adc| adc.dr_data());
            rprintln!("ADC DR: {}", dr_read.unwrap_or(0));
        }
        40 => {
            rprintln!("EXTI15_10");
            let exti = Exti::new();
            let pr = exti.pr_read(13);
            rprintln!("PR: {}", pr);
            exti.pr_clear(13);
        }
        _ => {
            ()
            // let exti = Exti::new();
            // exti.pr_clear(irqn as u8)
        }
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
