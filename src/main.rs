#![no_std]
#![no_main]
#![allow(unused_parens)]

use core::sync::atomic::{AtomicU32, Ordering};
// use cortex_m::asm::delay;
use cortex_m_rt::{entry, exception};
use panic_halt as _;

mod core_peripherals;
mod peripherals;
mod utils;

use crate::core_peripherals::scb::Scb;
use crate::peripherals::rcc::Rcc;
use crate::peripherals::stk::Stk;
use crate::peripherals::tim_gp::TimGp;
use core::cell::RefCell;

use cortex_m::interrupt::{self, Mutex};

use rtt_target::{rprintln, rtt_init_print};

static SYS_CLOCK: AtomicU32 = AtomicU32::new(0);
static PENDSV_COMMAND: Mutex<RefCell<Option<PendSVCommand>>> = Mutex::new(RefCell::new(None));

// PendSVCommand enum 정의
enum PendSVCommand {
    Log(&'static str),
    // Reset,
    // 다른 동작을 여기에 추가할 수 있습니다.
}

// trigger_command 함수 정의
fn trigger_pend_sv(command: PendSVCommand) {
    interrupt::free(|cs| {
        *PENDSV_COMMAND.borrow(cs).borrow_mut() = Some(command);
    });
    // PendSV 인터럽트를 트리거
    Scb::new().icsr_pendsvset_write();
}

// test_interrupt 함수 정의
// fn test_interrupt(num: u8) -> Result<(), &'static str> {
//     match num {
//         1 => Ok(()),
//         _ => {
//             trigger_pend_sv(PendSVCommand::Log("Invalid number"));
//             Err("Invalid number")
//         }
//     }
// }

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

    // 에러 발생 시 trigger_command 호출
    // let _ = test_interrupt(4);

    // TIM2
    
    
    let _ = match TimGp::new(2) {
        Ok(tim2) => {
            tim2.tim_gp_clock_enable();
            trigger_pend_sv(PendSVCommand::Log("TimGp Set"));

        },
        Err(_) => {
            trigger_pend_sv(PendSVCommand::Log("Error Invalid number"));
        },
    };
    
    loop {
        rprintln!("Hello, world!");
        // delay(SYS_CLOCK.load(Ordering::Acquire));
    }
}

#[exception]
fn PendSV() {
    interrupt::free(|cs| {
        if let Some(command) = PENDSV_COMMAND.borrow(cs).take() {
            match command {
                PendSVCommand::Log(message) => rprintln!("{}", message),
                // PendSVCommand::Reset => {
                //     rprintln!("System reset initiated.");
                //     // 시스템을 리셋하는 코드 (예시)
                //     // Scb::new().system_reset();
                // }
                // 다른 동작을 여기에 추가할 수 있습니다.
            }
        }
    });
}
