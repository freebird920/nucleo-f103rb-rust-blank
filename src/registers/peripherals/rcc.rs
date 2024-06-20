// src/registers/peripherals/rcc.rs

const RCC_BASE: u32 = 0x4002_1000;

pub struct Rcc {
    pub cr: *mut u32,
    pub cfgr: *mut u32,
    pub cir: *mut u32,
    pub apb2rstr: *mut u32,
    pub apb1rstr: *mut u32,
    pub ahbenr: *mut u32,
    pub apb2enr: *mut u32,
    pub apb1enr: *mut u32,
    pub bdcr: *mut u32,
    pub csr: *mut u32,
}

impl Rcc {
    pub fn new() -> Rcc {
        Rcc {
            cr: (RCC_BASE + 0x00) as *mut u32,
            cfgr: (RCC_BASE + 0x04) as *mut u32,
            cir: (RCC_BASE + 0x08) as *mut u32,
            apb2rstr: (RCC_BASE + 0x0C) as *mut u32,
            apb1rstr: (RCC_BASE + 0x10) as *mut u32,
            ahbenr: (RCC_BASE + 0x14) as *mut u32,
            apb2enr: (RCC_BASE + 0x18) as *mut u32,
            apb1enr: (RCC_BASE + 0x1C) as *mut u32,
            bdcr: (RCC_BASE + 0x20) as *mut u32,
            csr: (RCC_BASE + 0x24) as *mut u32,
        }
    }

    /// ## CR_HSION -  HSI ON
    /// HSI oscillator enabled
    pub fn cr_hsion(&self) {
        unsafe {
            let mut rcc_cr_val = self.cr.read_volatile();
            rcc_cr_val |= (1 << 0); // Enable HSI
            self.cr.write_volatile(rcc_cr_val);
            while self.cr.read_volatile() & (1 << 1) == 0 {} // Wait until HSI is ready
        }
    }


}

impl Rcc{
        /// ## apb2enr_iop_x_en_set
    /// iop_x_en bit set in APB2ENR register <br/>s
    /// ### @params
    /// - iop_x: u8 (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H)
    /// - val: u32 (0: disable, 1: enable)
    pub fn apb2enr_iop_x_en_set(&self, iop_x: u8, val: u32) -> Result<(), &'static str> {
        if val > 1 {
            return Err("invalid val. val: 0 | 1");
        };
        if iop_x > 7 {
            return Err("invalid iop_x. iop_x: 0 ~ 7");
        };
        let shift = iop_x + 2;
        unsafe {
            let mut apb2enr_val = self.apb2enr.read_volatile();
            apb2enr_val = apb2enr_val | (val << shift);
            self.apb2enr.write_volatile(apb2enr_val);
        }
        Ok(())
    }
}
