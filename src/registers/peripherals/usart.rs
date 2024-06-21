use super::rcc::Rcc;

pub struct Usart {
    usart_x: u8,
    sr: *mut u32,
    brr: *mut u32,
    cr1: *mut u32,
    cr2: *mut u32,
    cr3: *mut u32,
}

impl Usart {
    pub fn new(usart_x: u8) -> Result<Usart, &'static str> {
        let base_address = match usart_x {
            1 => 0x4001_3800,
            2 => 0x4000_4400,
            3 => 0x4000_4800,
            _ => 0x0,
        };

        Ok(
            Usart {
                usart_x,
            sr: (base_address + 0x00) as *mut u32,
            brr: (base_address + 0x08) as *mut u32,
            cr1: (base_address + 0x0C) as *mut u32,
            cr2: (base_address + 0x10) as *mut u32,
            cr3: (base_address + 0x14) as *mut u32,
            
        })
    }

    fn read_reg(&self, reg: *mut u32) -> u32 {
        unsafe { core::ptr::read_volatile(reg) }
    }

    fn write_reg(&self, reg: *mut u32, val: u32) {
        unsafe { core::ptr::write_volatile(reg, val) }
    }
}

impl Usart {
    pub fn clock_enable(&self) -> Result<(), &'static str>{
        let _ = match self.usart_x {
            2 => (Rcc::new().apb1enr_usart2en_set(1)),
            3 => (Rcc::new().apb1enr_usart3en_set(1)),
            _ => Err("Invalid usart_x"),
        };
        Ok(())
    }
    pub fn get_usart_x(&self) -> u8 {
        self.usart_x
    }
}
