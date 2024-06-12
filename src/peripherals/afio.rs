#![allow(non_snake_case)]


pub struct AFIO {
    base: u32,
}


pub enum EXTIx_Px {
    PA = 0b0000,
    PB = 0b0001,
    PC = 0b0010,
    PD = 0b0011,
    PE = 0b0100,
    PF = 0b0101,
    PG = 0b0110,
}

impl AFIO {
    pub fn new(base: u32) -> AFIO {
        AFIO {
            base: base,
        }
    }
    fn EVCR(&self) -> *mut u32 {
        (self.base) as *mut u32
    }
    fn MAPR(&self) -> *mut u32 {
        (self.base + 0x04) as *mut u32
    }
    fn EXTICR1(&self) -> *mut u32 {
        (self.base + 0x08) as *mut u32
    }
    fn EXTICR2(&self) -> *mut u32 {
        (self.base + 0x0C) as *mut u32
    }
    fn EXTICR3(&self) -> *mut u32 {
        (self.base + 0x10) as *mut u32
    }
    fn EXTICR4(&self) -> *mut u32 {
        (self.base + 0x14) as *mut u32
    }
    fn MAPR2(&self) -> *mut u32 {
        (self.base + 0x1C) as *mut u32
    }
    /// Configure the external interrupt line
    /// EXTIx external interrupt 
    pub fn exti_cr_x(&self, port: EXTIx_Px, pin: u8) {
        unsafe {
            let exticr = match pin {
                0..=3 => self.EXTICR1(),
                4..=7 => self.EXTICR2(),
                8..=11 => self.EXTICR3(),
                12..=15 => self.EXTICR4(),
                _ => self.EXTICR1(),
            };

            let mut exticr_val = exticr.read_volatile();
            let shift = (pin % 4) * 4;
            exticr_val &= !(0b1111 << shift);
            exticr_val |= (port as u32) << shift;
            // rprintln!( "EXTICR: {:#010b}", exticr_val);
            exticr.write_volatile(exticr_val);
        }

    }
}