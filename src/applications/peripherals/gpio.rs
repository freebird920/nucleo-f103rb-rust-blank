use super::rcc::UseRcc;
use crate::registers::peripherals::{gpio::Gpio, rcc::Rcc};

pub struct UseGpio<'life_use_gpio> {
    gpio: &'life_use_gpio Gpio,
}

impl<'life_use_gpio> UseGpio<'life_use_gpio> {
    pub fn new(gpio: &'life_use_gpio Gpio) -> UseGpio<'life_use_gpio> {
        let gpio_x = gpio.get_gpio_x();
        let _ = UseRcc::new(&Rcc::new()).abp2enr_iop_x_en_set(gpio_x, 1);
        UseGpio { gpio: gpio }
    }
}

impl<'life_use_gpio> UseGpio<'life_use_gpio> {
    /// ### cr_pin_config
    /// **cnf_mode** CNFy + MODEx
    /// ####CNFy: Port configuration bits
    /// ##### Input mode
    /// - 00: Analog mode
    /// - 01: Floating input (reset state)
    /// - 10: Input with pull-up / pull-down
    /// - 11: General purpose output push-pull
    /// ##### Output mode
    /// - 00: General purpose output push-pull
    /// - 01: General purpose output Open-drain
    /// - 10: Alternate function output Push-pull
    /// - 11: Alternate function output Open-drain
    /// #### MODEx
    /// - 00: Input mode (reset state)
    /// - 01: Output mode, max speed 10 MHz.
    /// - 10: Output mode, max speed 2 MHz.
    /// - 11: Output mode, max speed 50 MHz.
    pub fn pin_config(&self, pin: u8, cnf_mode: u32) -> Result<(), &'static str> {
        self.gpio.cr_set(pin, cnf_mode)?;
        Ok(())
    }
}


impl<'life_use_gpio> UseGpio<'life_use_gpio> { 
    pub fn port_bit_set(&self, pin: u8) -> Result<(),&'static str> {
        self.gpio.bsrr_bs(pin)?;
        Ok(())
    }
    pub fn port_bit_reset(&self, pin: u8) -> Result<(),&'static str> {
        self.gpio.bsrr_br(pin)?;
        Ok(())
    }
}