pub struct FLASH {
    base: u32,
}

pub enum FLASH_LATENCY {
    _0WS = 0b000,
    _1WS = 0b001,
    _2WS = 0b010,
}

impl FLASH {
    pub fn new(base: u32) -> FLASH {
        FLASH { base }
    }

    fn ACR(&self) -> *mut u32 {
        (self.base + 0x00) as *mut u32
    }

    pub fn ACR_PRFTBE(&self, enable: bool) {
        unsafe {
            let flash_acr = self.ACR();
            let mut flash_acr_val = flash_acr.read_volatile();
            if enable {
                flash_acr_val |= (1 << 4); // Enable prefetch buffer
            } else {
                flash_acr_val &= !(1 << 4); // Disable prefetch buffer
            }
            flash_acr.write_volatile(flash_acr_val);

            // Wait until prefetch buffer is enabled/disabled
            // 이 부분은 불필요할 수 있음. 대부분의 구현에서 해당 부분은 생략
        }
    }

    pub fn ACR_LATENCY(&self, flash_latency: FLASH_LATENCY) {
        unsafe {
            let flash_acr = self.ACR();
            let latency = flash_latency as u32;
            let mut flash_acr_val = flash_acr.read_volatile();
            flash_acr_val &= !(0b111 << 0);
            flash_acr_val |= (latency << 0);
            flash_acr.write_volatile(flash_acr_val);
        }
    }
}