use crate::registers::peripherals::usart::Usart;

struct UseUsart<'life_use_usart> {
    usart_x: u8,
    sr: *mut u32,
    usart: &'life_use_usart Usart,
}

impl<'life_use_usart> UseUsart<'life_use_usart> {
    pub fn new(usart: &'life_use_usart Usart) -> Result<UseUsart<'life_use_usart>, &'static str> {
        let usart_x = usart.get_usart_x();
        let _ = usart.clock_enable();
        let base_address: u32 = match usart_x {
            1 => 0x4001_3800,
            2 => 0x4000_4400,
            3 => 0x4000_4800,
            _ => return Err("UseUsart::new()  Err!! Invalid usart_x"),
        };
        Ok(UseUsart {
            usart_x,
            sr: (base_address + 0x00) as *mut u32,
            usart,
        })
    }
}
