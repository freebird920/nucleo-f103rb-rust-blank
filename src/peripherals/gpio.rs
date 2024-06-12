#![allow(non_snake_case)]
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
    pub fn idr_read(&self, port: u8) -> u32 {
        unsafe { (self.IDR().read_volatile() & (1 << port)) >> port }
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
    pub fn crh_port_config(&self, port: u8, mode: u32) {
        assert!(
            port >= 8 && port < 16,
            "Port number must be between 8 and 15 for CRH"
        );
        unsafe {
            let shift = (port - 8) * 4;
            let crh_reg = self.CRH();

            let mut cr_val = crh_reg.read_volatile();
            cr_val &= !(0b1111 << shift); // Clear the current configuration
            cr_val |= (mode << shift); // Set the mode
            crh_reg.write_volatile(cr_val);
        }
    }
}
