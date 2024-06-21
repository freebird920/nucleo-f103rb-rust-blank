
use crate::registers::peripherals::{gpio::Gpio, rcc::Rcc};
use super::rcc::UseRcc;

pub struct UseGpio<'life_use_gpio> {
    gpio: &'life_use_gpio Gpio,
}

impl<'life_use_gpio> UseGpio<'life_use_gpio> {
    pub fn new(gpio: &'life_use_gpio Gpio ) -> UseGpio<'life_use_gpio> {
        let gpio_x = gpio.get_gpio_x();
        let _ = UseRcc::new(&Rcc::new()).abp2enr_iop_x_en_set( gpio_x,1);
        UseGpio { gpio: gpio }
    }
}

// impl<'life_use_gpio> UseGpio<'life_use_gpio> {
//     pub fn clock_enable(&self) -> Result<(),&'static str> {
//         let gpio_x = self.gpio.get_gpio_x();
//         UseRcc::new(&Rcc::new()).abp2enr_iop_x_en_set( gpio_x,1)?;
//         Ok(())
//     }
// }

