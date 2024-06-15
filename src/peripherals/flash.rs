const FLASH_BASE: u32 = 0x4002_2000;

pub struct Flash {
    acr: *mut u32,
}

impl Flash {
    pub fn new() -> Flash {
        Flash {
            acr: (FLASH_BASE + 0x00) as *mut u32,
        }
    }



    pub fn acr_prftbe(&self, enable: bool) {
        unsafe {
            let mut flash_acr_val = self.acr.read_volatile();
            if enable {
                flash_acr_val |= (1 << 4); // Enable prefetch buffer
            } else {
                flash_acr_val &= !(1 << 4); // Disable prefetch buffer
            }
            self.acr.write_volatile(flash_acr_val);
        }
    }

    /// ## ACR_LATENCY
    /// Set flash latency
    /// - 0WS: 0b000 // System clock <= 24 MHz
    /// - 1WS: 0b001 // 24 MHz < System clock <= 48 MHz
    /// - 2WS: 0b010 // 48 MHz < System clock <= 72 MHz
    pub fn acr_latency(&self, flash_latency: u8) {
        if flash_latency > 2 {
            panic!("Flash latency must be 0, 1, or 2");
        }
        unsafe {
            let latency = flash_latency as u32;
            let mut flash_acr_val = self.acr.read_volatile();
            flash_acr_val &= !(0b111 << 0);
            flash_acr_val |= (latency << 0);
            self.acr.write_volatile(flash_acr_val);
        }
    }
}
