const BASE_EXTI : u32 = 0x4001_0400;
pub struct Exti {
    // base: u32,
    imr:    *mut u32,
    // emr:    *mut u32,
    rtsr:   *mut u32,
    ftsr:   *mut u32,
    // swier:  *mut u32,
    pr:     *mut u32,
}


impl Exti {
    pub fn new() -> Exti {
        let base = BASE_EXTI;
        Exti {
            // base,
            imr:    (base + 0x00) as *mut u32,
            // emr:    (base + 0x04) as *mut u32,
            rtsr:   (base + 0x08) as *mut u32,
            ftsr:   (base + 0x0C) as *mut u32,
            // swier:  (base + 0x10) as *mut u32,
            pr:     (base + 0x14) as *mut u32,
        }
    }
    

    pub fn imr_set(&self, mr_x: u8, enable: bool)-> Result<(), &'static str>{
        match mr_x {
            0..=19  => (),
            _       => Err("Invalid mr_x")?, 
        }

        
        unsafe {
            let mut imr_val = self.imr.read_volatile();
            if enable {
                imr_val |= 1 << mr_x;
            } else {
                imr_val &= !(1 << mr_x);
            }
            self.imr.write_volatile(imr_val);
        }
        Ok(())
    }


    pub fn rtsr_set(&self, tr_x: u8, val: bool) {
        unsafe {
            let mut rtsr_val = self.rtsr.read_volatile();
            if val {
                rtsr_val |= 1 << tr_x;
            } else {
                rtsr_val &= !(1 << tr_x);
            }
            self.rtsr.write_volatile(rtsr_val);
        }
    }



    pub fn ftsr_set(&self, tr_x: u8, enable: bool) {
        unsafe {
            let mut ftsr_val = self.ftsr.read_volatile();
            if enable {
                ftsr_val |= 1 << tr_x;
            } else {
                ftsr_val &= !(1 << tr_x);
            }
            self.ftsr.write_volatile(ftsr_val);
        }
    }


    /// EXTI_PR Pending register
    pub fn pr_read(&self, pr_x: u8) -> bool {
        unsafe {
            let pr_val = self.pr.read_volatile();
            return pr_val & (1 << pr_x) != 0

        }
    }
    
    /// EXTI_PR Pending register
    pub fn pr_clear(&self, pr_x: u8) {
        unsafe {
            let mut pr_val = self.pr.read_volatile();
            pr_val |= (1 << pr_x);
            self.pr.write_volatile(pr_val);
        }
    }

}