#![allow(non_snake_case)]
pub struct EXTI {
    base: u32,
}

impl EXTI {
    pub fn new(base: u32) -> EXTI {
        EXTI {
            base: base,
        }
    }
    fn IMR(&self) -> *mut u32 {
        (self.base) as *mut u32
    }
    fn EMR(&self) -> *mut u32 {
        (self.base + 0x04) as *mut u32
    }
    fn RTSR(&self) -> *mut u32 {
        (self.base + 0x08) as *mut u32
    }
    fn FTSR(&self) -> *mut u32 {
        (self.base + 0x0C) as *mut u32
    }
    fn SWIER(&self) -> *mut u32 {
        (self.base + 0x10) as *mut u32
    }
    fn PR(&self) -> *mut u32 {
        (self.base + 0x14) as *mut u32
    }
    pub fn imr_set(&self, MRx: u8, enable: bool) {
        unsafe {
            let imr = self.IMR();
            let mut imr_val = imr.read_volatile();
            if enable {
                imr_val |= 1 << MRx;
            } else {
                imr_val &= !(1 << MRx);
            }
            imr.write_volatile(imr_val);
        }
    }
    pub fn rstr_set(&self, TRx: u8, val: bool) {
        unsafe {
            let rtsr = self.RTSR();
            let mut rtsr_val = rtsr.read_volatile();
            if val {
                rtsr_val |= 1 << TRx;
            } else {
                rtsr_val &= !(1 << TRx);
            }
            rtsr.write_volatile(rtsr_val);
        }
    }
    pub fn rtsr_set(&self, TRx: u8, val: bool) {
        unsafe {
            let rtsr = self.RTSR();
            let mut rtsr_val = rtsr.read_volatile();
            if val {
                rtsr_val |= 1 << TRx;
            } else {
                rtsr_val &= !(1 << TRx);
            }
            rtsr.write_volatile(rtsr_val);
        }
    }


    pub fn ftsr_set(&self, TRx: u8, val: bool) {
        unsafe {
            let ftsr = self.FTSR();
            let mut ftsr_val = ftsr.read_volatile();
            if val {
                ftsr_val |= 1 << TRx;
            } else {
                ftsr_val &= !(1 << TRx);
            }
            ftsr.write_volatile(ftsr_val);
        }
    }

    pub fn pr_read(&self, PRx: u8) -> bool {
        unsafe {
            let pr = self.PR();
            let pr_val = pr.read_volatile();
            pr_val & (1 << PRx) != 0
        }
    }

}