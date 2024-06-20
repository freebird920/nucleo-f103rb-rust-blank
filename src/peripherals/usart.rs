use super::rcc::Rcc;

struct Usart {
    usart_x: u8,
    base_address: u32,

}

impl Usart {
    pub fn new(usart_x: u8) -> Result<Usart, &'static str> {
        let base_address: Result<u32, &'static str> = match usart_x {
            2 => Ok(0x4000_4400),
            3 => Ok(0x4000_4800),
            4 => Ok(0x4000_4C00),
            5 => Ok(0x4000_5000),
            _ => Err("Invalid USART number"),
        };
        Ok(Usart {
            usart_x,
            base_address: base_address?,
        })
    }
    pub fn usart_clock_enable(&self) {
        let rcc = Rcc::new();
    }
}
