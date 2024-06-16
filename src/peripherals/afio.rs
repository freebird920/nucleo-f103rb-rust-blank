const BASE_AFIO: u32 = 0x4001_0000;
pub enum EXTIx_Px {
    PA = 0b0000,
    PB = 0b0001,
    PC = 0b0010,
    PD = 0b0011,
    PE = 0b0100,
    PF = 0b0101,
    PG = 0b0110,
}
pub struct AFIO {
    // base: u32,
    evcr: *mut u32,
}

impl AFIO {
    pub fn new() -> AFIO {
        let base_addr: u32 = BASE_AFIO;
        AFIO {
            // base: base_addr,
            evcr: (base_addr + 0x00) as *mut u32,
        }
    }

    /// ### EXTIx Configuration Register
    /// ##### port: EXTIx_Px
    /// ##### pin: 0~15
    /// #### port number
    /// - 0: PA
    /// - 1: PB
    /// - 2: PC
    /// - 3: PD
    /// - 4: PE
    /// - 5: PF
    /// - 6: PG
    pub fn exti_cr(&self, port: u32, pin: u32) -> Result<(), &'static str> {
        if !(0..=6).contains(&port) {
            return Err("Invalid port number");
        }
        if !(0..=15).contains(&pin) {
            return Err("Invalid pin number");
        }
    
        let addr_offset = 0x08 + (pin / 4) * 4;
        unsafe {
            let exticr = (BASE_AFIO + addr_offset) as *mut u32;
            let mut exticr_val = exticr.read_volatile();
            let shift = (pin % 4) * 4;
            exticr_val &= !(0b1111 << shift);
            exticr_val |= (port as u32) << shift;
            exticr.write_volatile(exticr_val);
        }
    
        Ok(())
    }
    

    // fn EVCR(&self) -> *mut u32 {
    //     (self.base) as *mut u32
    // }
    // fn MAPR(&self) -> *mut u32 {
    //     (self.base + 0x04) as *mut u32
    // }
    // fn EXTICR1(&self) -> *mut u32 {
    //     (self.base + 0x08) as *mut u32
    // }
    // fn EXTICR2(&self) -> *mut u32 {
    //     (self.base + 0x0C) as *mut u32
    // }
    // fn EXTICR3(&self) -> *mut u32 {
    //     (self.base + 0x10) as *mut u32
    // }
    // fn EXTICR4(&self) -> *mut u32 {
    //     (self.base + 0x14) as *mut u32
    // }
    // fn MAPR2(&self) -> *mut u32 {
    //     (self.base + 0x1C) as *mut u32
    // }
    // /// Configure the external interrupt line
    // /// EXTIx external interrupt
    // pub fn exti_cr_x(&self, port: EXTIx_Px, pin: u8) {
    //     unsafe {
    //         let exticr = match pin {
    //             0..=3 => self.EXTICR1(),
    //             4..=7 => self.EXTICR2(),
    //             8..=11 => self.EXTICR3(),
    //             12..=15 => self.EXTICR4(),
    //             _ => self.EXTICR1(),
    //         };

    //         let mut exticr_val = exticr.read_volatile();
    //         let shift = (pin % 4) * 4;
    //         exticr_val &= !(0b1111 << shift);
    //         exticr_val |= (port as u32) << shift;
    //         // rprintln!( "EXTICR: {:#010b}", exticr_val);
    //         exticr.write_volatile(exticr_val);
    //     }

    // }
}
