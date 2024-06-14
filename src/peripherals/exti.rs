#![allow(non_snake_case)]
pub struct EXTI {
    base: u32,
    imr:    *mut u32,
    emr:    *mut u32,
    rtsr:   *mut u32,
    ftsr:   *mut u32,
    swier:  *mut u32,
    pr:     *mut u32,
}


impl EXTI {
    pub fn new(base: u32) -> EXTI {
        EXTI {
            base,
            imr:    (base + 0x00) as *mut u32,
            emr:    (base + 0x04) as *mut u32,
            rtsr:   (base + 0x08) as *mut u32,
            ftsr:   (base + 0x0C) as *mut u32,
            swier:  (base + 0x10) as *mut u32,
            pr:     (base + 0x14) as *mut u32,
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
            let mut imr_val = self.imr.read_volatile();
            if enable {
                imr_val |= 1 << MRx;
            } else {
                imr_val &= !(1 << MRx);
            }
            self.imr.write_volatile(imr_val);
        }
    }
    pub fn rstr_set(&self, TRx: u8, val: bool) {
        unsafe {
            let mut rtsr_val = self.rtsr.read_volatile();
            if val {
                rtsr_val |= 1 << TRx;
            } else {
                rtsr_val &= !(1 << TRx);
            }
            self.rtsr.write_volatile(rtsr_val);
        }
    }
    pub fn rtsr_set(&self, TRx: u8, val: bool) {
        unsafe {
            let mut rtsr_val = self.rtsr.read_volatile();
            if val {
                rtsr_val |= 1 << TRx;
            } else {
                rtsr_val &= !(1 << TRx);
            }
            self.rtsr.write_volatile(rtsr_val);
        }
    }


    pub fn ftsr_set(&self, TRx: u8, val: bool) {
        unsafe {
            let mut ftsr_val = self.ftsr.read_volatile();
            if val {
                ftsr_val |= 1 << TRx;
            } else {
                ftsr_val &= !(1 << TRx);
            }
            self.ftsr.write_volatile(ftsr_val);
        }
    }
    /// EXTI_PR Pending register
    pub fn pr_read(&self, PRx: u8) -> bool {
        unsafe {
            let pr_val = self.pr.read_volatile();
            return pr_val & (1 << PRx) != 0

        }
    }
    
    /// EXTI_PR Pending register
    pub fn pr_clear(&self, PRx: u8) {
        unsafe {
            let mut pr_val = self.pr.read_volatile();
            pr_val |= (1 << PRx);
            self.pr.write_volatile(pr_val);
        }
    }

}