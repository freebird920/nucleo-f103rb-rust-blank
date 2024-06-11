pub enum GPIOx_BASE {
    A = 0x4001_0800,
    B = 0x4001_0C00,
    C = 0x4001_1000,
}

pub struct GPIO {
    base: u32,
}

impl GPIO {
    pub fn new(base: GPIOx_BASE) -> Self {
        GPIO { base: base as u32 }
    }

    unsafe fn CRL(&self) -> *mut u32 {
        (self.base + 0x00) as *mut u32
    }

    unsafe fn CRH(&self) -> *mut u32 {
        (self.base + 0x04) as *mut u32
    }

    unsafe fn IDR(&self) -> *mut u32 {
        (self.base + 0x08) as *mut u32
    }

    unsafe fn BSRR(&self) -> *mut u32 {
        (self.base + 0x10) as *mut u32
    }

    pub fn bsrr_write(&self, port: u8) {
        unsafe {
            self.BSRR().write_volatile(1 << port);
        }
    }

    pub fn bsrr_reset(&self, port: u8) {
        unsafe {
            self.BSRR().write_volatile(1 << (port + 16));
        }
    }
    pub fn crl_port_config(&self, port: u8, cnf_mode: u32) {
        assert!(port < 8, "Port number must be between 0 and 7");
        unsafe {
            let shift = port * 4;
            let mut crl_val = self.CRL().read_volatile();
            crl_val &= !(0b1111 << shift); // Clear the current configuration
            crl_val |= (cnf_mode << shift); // Set the configuration
            self.CRL().write_volatile(crl_val);
        }
    }
    pub fn crh_port_config(&self, port: u8, cnf_mode: u32) {
        assert!(port < 8, "Port number must be between 8 and 15");
        let shift = (port - 8) * 4;
        unsafe {
            let mut crh_val = self.CRH().read_volatile();
            crh_val &= !(0b1111 << shift); // Clear the current configuration
            crh_val |= (cnf_mode << shift); // Set the configuration
            self.CRH().write_volatile(crh_val);
        }
    }

}