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
    // new()
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
}

impl Rcc {
    fn read_reg(&self, reg: *mut u32) -> u32 {
        unsafe { core::ptr::read_volatile(reg) }
    }

    fn write_reg(&self, reg: *mut u32, val: u32) {
        unsafe { core::ptr::write_volatile(reg, val) }
    }
}

// ## CR
impl Rcc {

    /// ## CR[24] PLLON CR[25] PLLRDY -  PLL ON
    pub fn cr_pllon(&self) {
        let mut rcc_cr_val = self.read_reg(self.cr);
        rcc_cr_val |= (1 << 24); // Enable PLL
        self.write_reg(self.cr, rcc_cr_val);
        while self.read_reg(self.cr) & (1 << 25) == 0 {} // Wait until PLL is ready
    }

    /// ## CR[0] HSION -  HSI ON
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

// ## CFGR
impl Rcc {
    /// ### CFGR[21:18] PLLMUL - PLL multiplication factor
    /// **IMPORTANT** PLL output freq muts not exceed 72MHz
    /// - 0000: PLL input clock x 2
    /// - 0001: PLL input clock x 3
    /// - 0010: PLL input clock x 4
    /// - 0011: PLL input clock x 5
    /// - 0100: PLL input clock x 6
    /// - 0101: PLL input clock x 7
    /// - 0110: PLL input clock x 8
    /// - 0111: PLL input clock x 9
    /// - 1000: PLL input clock x 10
    /// - 1001: PLL input clock x 11
    /// - 1010: PLL input clock x 12
    /// - 1011: PLL input clock x 13
    /// - 1100: PLL input clock x 14
    /// - 1101: PLL input clock x 15
    /// - 1110: PLL input clock x 16
    /// - 1111: PLL input clock x 16
    pub fn cfgr_pllmul_set(&self, multiplication_factor: u32) -> Result<(), &'static str> {
        if (multiplication_factor > 0b1111) {
            return Err("invalid multiplication_factor");
        };

        let mut rcc_cfgr_val = self.read_reg(self.cfgr);
        rcc_cfgr_val &= !(0b1111 << 18); // Clear PLLMUL bits
        rcc_cfgr_val |= (multiplication_factor << 18); // Set PLLMUL bits
        self.write_reg(self.cfgr, rcc_cfgr_val);
        Ok(())
    }

    /// ### CFGR[16] PLLSRC - PLL entry clock source
    /// - 0: HSI/2 selected as PLL input clock
    /// - 1: HSE selected as PLL input clock
    pub fn cfgr_pllsrc_set(&self, pllsrc: u32) -> Result<(), &'static str> {
        let mut rcc_cfgr_val = self.read_reg(self.cfgr);
        match pllsrc {
            0 => rcc_cfgr_val &= !(0b1 << 16),   // Clear PLLSRC bit
            1 => rcc_cfgr_val |= (pllsrc << 16), // Set PLLSRC bit
            _ => return Err("invalid pllsrc"),
        };
        self.write_reg(self.cfgr, rcc_cfgr_val);
        Ok(())
    }
    /// ### CFGR[0] SW - System clock switch
    /// - 0b00: HSI selected as system clock
    /// - 0b01: HSE selected as system clock
    /// - 0b10: PLL selected as system clock
    /// - 0b11: not allowed
    ///
    pub fn cfgr_sw_set(&self, sw: u32) -> Result<(), &'static str> {
        let mut rcc_cfgr_val = self.read_reg(self.cfgr);
        rcc_cfgr_val &= !(0b11 << 0); // Clear SW bits
        match sw {
            0 => (),
            1 => rcc_cfgr_val |= (sw << 0),
            2 => rcc_cfgr_val |= (sw << 0), // Set SW bits
            _ => return Err("invalid sw"),
        };
        self.write_reg(self.cfgr, rcc_cfgr_val);
        Ok(())
    }
}

// ## APB2ENR
impl Rcc {
    // apb2enr
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
        let mut apb2enr_val = self.read_reg(self.apb2enr);
        apb2enr_val = apb2enr_val | (val << shift);
        self.write_reg(self.apb2enr, apb2enr_val);
        Ok(())
    }


}
impl Rcc{
        /// # APB1ENR
        
        /// ## USART2EN[17]
        pub fn apb1enr_usart2en_set(&self, val:u32) -> Result<(), &'static str>{
            if(val > 1) {return Err("usart2en_set(): invalid val. val: 0 | 1");}
            
            let mut apb1enr_val = self.read_reg(self.apb1enr);
            apb1enr_val |= (1 << 17); // Enable USART2
            self.write_reg(self.apb1enr, apb1enr_val);
            Ok(())
        }
        pub fn apb1enr_usart3en_set(&self, val:u32) -> Result<(), &'static str>{
            if(val > 1) {return Err("usart3en_set(): invalid val. val: 0 | 1");}
            
            let mut apb1enr_val = self.read_reg(self.apb1enr);
            apb1enr_val |= (1 << 18); // Enable USART3
            self.write_reg(self.apb1enr, apb1enr_val);
            Ok(())
        }
}