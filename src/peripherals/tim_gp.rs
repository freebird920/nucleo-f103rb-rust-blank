pub enum TIM_GP_TYPE {
    TIM2 = 0x4000_0000,
    TIM5 = 0x4000_0C00 
}

const TIM2_BASE: u32 = 0x4000_0000;

pub struct TIM_GP{
    base: u32,
}

impl TIM_GP {
    pub fn new(tim_gp: TIM_GP_TYPE) -> Self {
        TIM_GP { base: tim_gp as u32 }
    }

    unsafe fn CR1(&self) -> *mut u32 {
        (self.base + 0x00) as *mut u32
    }
    unsafe fn PSC(&self) -> *mut u32 {
        (self.base + 0x28) as *mut u32
    }
    pub fn cr1_cen_set(&self, enable: bool) {
        unsafe {
            let mut cr1_val = self.CR1().read_volatile();
            if enable {
                cr1_val |=  (0b1 << 0)
            } else {
                cr1_val &= !(0b1 << 0)
            }
        }
    }
    pub fn set_psc (&self, psc_value: u32) {
        unsafe {
            self.PSC().write_volatile(psc_value);
        }
    }

}

pub struct TIM2 {
    base: u32,
}

impl TIM2 {
    pub fn new() -> TIM2 {
        TIM2 { base: TIM2_BASE }
    }

    fn CR1(&self) -> *mut u32 {
        (self.base + 0x00) as *mut u32
    }

    fn PSC(&self) -> *mut u32 {
        (self.base + 0x28) as *mut u32
    }

    fn ARR(&self) -> *mut u32 {
        (self.base + 0x2C) as *mut u32
    }

    fn SR(&self) -> *mut u32 {
        (self.base + 0x10) as *mut u32
    }

    fn CNT(&self) -> *mut u32 {
        (self.base + 0x24) as *mut u32
    }

    pub unsafe fn init(&self, psc: u32, arr: u32) {
        self.PSC().write_volatile(psc);
        self.ARR().write_volatile(arr);
        self.CR1().write_volatile(0x1); // Enable TIM2
    }

    pub unsafe fn delay_ms(&self, ms: u32) {
        let target = ms; // Convert ms to us
        self.CNT().write_volatile(0); // Reset counter
        while self.CNT().read_volatile() < target {}
    }
}
