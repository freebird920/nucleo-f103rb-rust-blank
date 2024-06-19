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

use crate::external::pcf8574::Pcf8574;
use crate::external::hd44780::Hd44780;
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

    // ADC 세팅
    rprintln!("abp2enr: {}", rcc.apb2enr_read());

    let adc = Adc::new(1)
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("ADC Set")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));
    let _ = adc.as_ref().map(|adc| {
        rcc.enable_adc1();
        adc.cr2_adon(true); // ADC ON
        delay(14);
        adc.cr1_scan(true); // 스캔 모드 활성화
                            // adc.cr1_eocie_set(true); // EOC 인터럽트 활성화

        adc.cr2_extsel(111); // Software start
        adc.cr2_cont(true); // 연속 변환모드

        adc.cr2_cal(); // 보정
        rprintln!("cr1_read {}", adc.cr1_read());
        rprintln!("cr2_read {}", adc.cr2_read());

        let _ = adc.smpr_smp(10, 0b101); // 채널 10 샘플링 시간 설정
        let _ = adc.smpr_smp(11, 0b101); // 채널 11 샘플링 시간 설정
        adc.sqr3_sq(1, 10); // 첫 번째 변환을 채널 10으로 설정
        adc.sqr3_sq(2, 11); // 두 번째 변환을 채널 11으로 설정
        let _ = adc.sqr_l(1); // 두 개의 변환을 수행하도록 설정

        rprintln!("ADC 설정 완료");

        // 변환 시작
        adc.cr2_swstart(true); // 변환 시작

        // 첫 번째 변환 (PC0)
        while !adc.sr_eoc_read() {
            // 변환 완료 대기
        }
        rprintln!("ADC DR (PC0): {}", adc.dr_data());

        // 두 번째 변환 (PC1)
        while !adc.sr_eoc_read() {
            // 변환 완료 대기
        }
        rprintln!("ADC DR (PC1): {}", adc.dr_data());
    });
    rprintln!("abp1enr: {}", rcc.apb1enr_read());
    // I2c 초기화
    let i2c2 = I2c::new(2)
        .inspect(|_| trigger_pend_sv(PendSVCommand::Log("I2C2 Init")))
        .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e)));
    let _ = i2c2.as_ref().ok().map(|i2c| {
        rprintln!("abp1enr: {}", rcc.apb1enr_read());

        i2c.i2c_clock_enable(); // enable I2C2 clock
        i2c.cr1_pe(false); // disable I2C2 - 먼저 I2C2를 비활성화 한 후 설정 시작
        let _ = i2c
            .cr2_freq(8)
            .inspect(|_| trigger_pend_sv(PendSVCommand::Log(" cr2_freq good")))
            .inspect_err(|e| trigger_pend_sv(PendSVCommand::Log(e))); // 32MHz 클럭 설정
        i2c.ccr_set_std(); // 표준 모드 설정 + trise 설정
        i2c.cr1_pe(true); // enable I2C2 - 설정 완료 후 I2C2 활성화
    });
    // send adta
    let _ = i2c2.as_ref().map(|i2c| {
        let hd44780_address: u8 = 0b100111;
        let hd44780 = Hd44780::new(hd44780_address, i2c); 
        let _ = hd44780.send_address();
        delay(32 * 1000 * 1000);
        rprintln!("1");
    
        let _ = hd44780.send_cmd(0b1100_0010);
        delay(32 * 1000 * 1000);
        rprintln!("2");
    
        let _ = hd44780.send_cmd(0b0010_1000);
        delay(32 * 1000 * 1000);
        rprintln!("3");
    
        let _ = hd44780.send_cmd(0b1000_1100);
        delay(32 * 1000 * 1000);
        rprintln!("4");
    
        let _ = hd44780.send_cmd(0b1000_1000);
        delay(32 * 1000 * 1000);
        rprintln!("5");
    
        let _ = hd44780.send_cmd(0b0000_1100);
        delay(32 * 1000 * 1000);
        rprintln!("6");
    
        let _ = hd44780.send_cmd(0b0000_1000);
        delay(32 * 1000 * 1000);
        rprintln!("7");
    
        let _ = hd44780.send_cmd(0b0001_1100);
        delay(32 * 1000 * 1000);
        rprintln!("8");
    
        let _ = hd44780.send_cmd(0b0001_1000);
        delay(32 * 1000 * 1000);
        delay(32 * 1000 * 1000);
        rprintln!("9");
    
        let _ = hd44780.send_cmd(0b01001000); // 'H'
        delay(32 * 1000 * 1000);
        rprintln!("10");
    
        let _ = hd44780.send_cmd(0b01000101); // 'E'
        delay(32 * 1000 * 1000);
        rprintln!("11");
    
        let _ = hd44780.send_cmd(0b01001100); // 'L'
        delay(32 * 1000 * 1000);
        rprintln!("12");
    
        let _ = hd44780.send_cmd(0b01001100); // 'L'
        delay(32 * 1000 * 1000);
        rprintln!("13");
    
        let _ = hd44780.send_cmd(0b01001111); // 'O'
        delay(32 * 1000 * 1000);
        rprintln!("14");
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
