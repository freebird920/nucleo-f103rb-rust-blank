use crate::peripherals::flash::{FLASH, FLASH_LATENCY};

pub struct RCC {
    base: u32,
}
pub enum TIMxEN {
    TIM2EN = 0,
    TIM3EN = 1,
    TIM4EN = 2,
    TIM5EN = 3,
    TIM6EN = 4,
    TIM7EN = 5,
}
pub enum IOPxEN {
    IOPAEN = 2,
    IOPBEN = 3,
    IOPCEN = 4,
    IOPDEN = 5,
    IOPEEN = 6,
}

#[allow(non_snake_case)]
impl RCC {
    pub fn new(base: u32) -> RCC {
        RCC { base }
    }

    fn CR(&self) -> *mut u32 {
        (self.base + 0x00) as *mut u32
    }

    fn CFGR(&self) -> *mut u32 {
        (self.base + 0x04) as *mut u32
    }

    pub fn CR_HSION(&self) {
        unsafe {
            let rcc_cr = self.CR();
            let mut rcc_cr_val = rcc_cr.read_volatile();
            rcc_cr_val |= (1 << 0); // Enable HSI
            rcc_cr.write_volatile(rcc_cr_val);
            while rcc_cr.read_volatile() & (1 << 1) == 0 {} // Wait until HSI is ready
        }
    }
    pub fn read_cfgr(&self) -> u32 {
        unsafe {
            let rcc_cfgr = self.CFGR();
            rcc_cfgr.read_volatile()
        }
    }
    pub fn read_cr(&self) -> u32 {
        unsafe {
            let rcc_cr = self.CR();
            rcc_cr.read_volatile()
        }
    }

    pub fn set_sys_clock_32MHz(&self) {
        unsafe {
            let rcc_cr = self.CR();
            let mut rcc_cr_val = rcc_cr.read_volatile();
            rcc_cr_val |= (1 << 0); // HSION
            rcc_cr.write_volatile(rcc_cr_val);
            while (rcc_cr.read_volatile() & (1 << 1)) == 0 {} // Wait until HSIRDY

            let rcc_cfgr = self.CFGR();
            let mut rcc_cfgr_val = rcc_cfgr.read_volatile();
            rcc_cfgr_val &= !(1 << 16); // Clear PLLSRC bit (select HSI/2)
            rcc_cfgr_val &= !(0b1111 << 18); // Clear PLLMUL bits
            rcc_cfgr_val |= (0b0110 << 18); // Set PLLMUL to 8 (4 MHz * 8 = 32 MHz)
            rcc_cfgr.write_volatile(rcc_cfgr_val);

            rcc_cfgr_val = rcc_cfgr.read_volatile();
            rcc_cfgr_val &= !(0b11 << 0); // Clear SW bits
            rcc_cfgr_val |= (0b10 << 0); // Set SW to 0b10 (PLL selected as system clock)
            rcc_cfgr.write_volatile(rcc_cfgr_val);

            rcc_cr_val = rcc_cr.read_volatile();
            rcc_cr_val |= (1 << 24); // PLLON
            rcc_cr.write_volatile(rcc_cr_val);
            while (rcc_cr.read_volatile() & (1 << 25)) == 0 {} // Wait until PLLRDY

            while (rcc_cfgr.read_volatile() & (0b11 << 2)) != (0b10 << 2) {} // Wait until SWS is PLL
            let flash = FLASH::new(0x4002_2000);
            flash.ACR_LATENCY(FLASH_LATENCY::_1WS); // Set flash latency
            flash.ACR_PRFTBE(true); // Enable prefetch buffer
        }
    }
    fn CR_PLLON(&self) {
        unsafe {
            let rcc_cr = self.CR();
            let mut rcc_cr_val = rcc_cr.read_volatile();
            rcc_cr_val |= (1 << 24); // Enable PLL
            rcc_cr.write_volatile(rcc_cr_val);
            while rcc_cr.read_volatile() & (1 << 25) == 0 {} // Wait until PLL is ready
        }
    }

    fn APB2ENR(&self) -> *mut u32 {
        (self.base + 0x18) as *mut u32
    }

    pub fn APB2ENR_IOPx_EN(&self, iop_x_en: IOPxEN, enable: bool) {
        unsafe {
            let apb2enr = self.APB2ENR();
            let mut apb2enr_val = apb2enr.read_volatile();
            let bit = iop_x_en as u32;
            if enable {
                apb2enr_val |= (1 << bit); // Enable IOPx
            } else {
                apb2enr_val &= !(1 << bit); // Disable IOPx
            }
            apb2enr.write_volatile(apb2enr_val);
        }
    }
    fn APB1ENR(&self) -> *mut u32 {
        (self.base + 0x1C) as *mut u32
    }
    pub fn APB1ENR_TIMxEN(&self, tim_x_en: TIMxEN, enable: bool) {
        unsafe {
            let apb1enr = self.APB1ENR();
            let mut apb1enr_val = apb1enr.read_volatile();
            let bit = tim_x_en as u32;
            if enable {
                apb1enr_val |= (1 << bit); // Enable TIMx
            } else {
                apb1enr_val &= !(1 << bit); // Disable TIMx
            }
            apb1enr.write_volatile(apb1enr_val);
        }
    }
    pub fn APB1ENR_TIM2EN(&self, enable: bool) {
        unsafe {
            let apb1enr = self.APB1ENR();
            let mut apb1enr_val = apb1enr.read_volatile();
            if enable {
                apb1enr_val |= (1 << 0); // Enable TIM2
            } else {
                apb1enr_val &= !(1 << 0); // Disable TIM2
            }
            apb1enr.write_volatile(apb1enr_val);
        }
    }
}
