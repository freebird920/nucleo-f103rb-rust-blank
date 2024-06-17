use super::rcc::Rcc;

const BASE_AFIO: u32 = 0x4001_0000;
pub struct Afio {
    base: u32,
    // evcr: *mut u32,
}

impl Afio {
    pub fn new() -> Afio {
        let base_addr: u32 = BASE_AFIO;
        Afio {
            base: base_addr,
            // evcr: (base_addr + 0x00) as *mut u32,
        }
    }

    /// ### afio_exticr_read
    /// **exti_cr_x** 1~4
    pub fn afio_exticr_read(&self, exti_cr_x: u8) ->Result<u32, &'static str>{
        match exti_cr_x {
            1..=4 => (),
            _ => return Err("Invalid exti_cr_x"),
        }
        unsafe {
            let exticr = (self.base + 0x08 + ((exti_cr_x-1) as u32 * 4)) as *const u32;
            Ok(exticr.read_volatile())

        }
    }
    pub fn afio_clock_enable(&self) {
        // RCC_APB2ENR
        Rcc::new().abp2enr_afioen(true);
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
    

}
