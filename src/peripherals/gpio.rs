#![allow(non_snake_case)]
pub enum GpioXBase {
    A = 0x4001_0800,
    B = 0x4001_0C00,
    C = 0x4001_1000,
}

pub struct Gpio {
    base: u32,
}

impl Gpio {
    pub fn new(base: GpioXBase) -> Self {
        Gpio { base: base as u32 }
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

    
    /// ## crl_port_config                                  
    /// cnf_mode: CNFy + MODEx                              <br/>
    /// #### CNFy: Port configuration bits (y = 0..7)       
    /// **00**: Analog mode                                 <br/>
    /// **01**: Floating input (reset state)                <br/>
    /// **10**: Input with pull-up / pull-down              <br/>
    /// **11**: General purpose output push-pull            <br/>     
    /// #### MODEx                                          
    /// **00**: Input mode (reset state)                    <br/>
    /// **01**: Output mode, max speed 10 MHz.              <br/>
    /// **10**: Output mode, max speed 2 MHz.               <br/>
    /// **11**: Output mode, max speed 50 MHz.              <br/>
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
    pub fn configure_pc2_as_analog(&self) {
        unsafe {
            let gpio_c_crl = 0x4001_1000 as *mut u32; // GPIOC_CRL 레지스터 주소
            let mut moder = gpio_c_crl.read_volatile();
            moder &= !(0b11 << (2 * 4));  // CNF2[1:0] = 00 (아날로그 모드)
            moder &= !(0b11 << ((2 * 4) + 2));  // MODE2[1:0] = 00 (입력 모드)
            gpio_c_crl.write_volatile(moder);
        }
    }

}
