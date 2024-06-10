const TIM2_BASE: u32 = 0x4000_0000;

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
