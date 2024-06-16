#![allow(non_snake_case)]

use crate::peripherals::rcc::Rcc;
pub struct Gpio {
    // base: u32,
    gpio_x : u8,
    crl: *mut u32 ,
    crh: *mut u32,
    bsrr: *mut u32,
    idr: *mut u32,
}

impl Gpio {
    /// ### Gpio::new
    /// **gpio_x** 
    /// - 0: GPIOA
    /// - 1: GPIOB
    /// - 2: GPIOC
    pub fn new(gpio_x: u8) -> Result<Gpio, &'static str> {

        let base: Result<u32, &'static str> = match gpio_x {
            0 => Ok(0x4001_0800),
            1 => Ok(0x4001_0C00),
            2 => Ok(0x4001_1000),
            _ => Err(("Invalid GPIO port")),
        };
        let base_value = match base {
            Ok(value) => value,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(Gpio {
            // base: base_value,
            gpio_x: gpio_x,
            crl: (base_value + 0x00) as *mut u32,
            crh: (base_value + 0x04) as *mut u32,
            bsrr: (base_value + 0x10) as *mut u32,
            idr: (base_value + 0x08) as *mut u32,
        })
    }

    /// ### cr_x
    /// **pin** pin은 0 ~ 15까지의 값을 가질 수 있습니다.
    pub fn cr_x (&self, pin: u8) -> Result<*mut u32,&'static str>{
        match pin {
            0..=7 => Ok(self.crl),
            8..=15 => Ok(self.crh),
            _ => Err("Invalid pin number"),
        }
    }

    pub fn gpio_clock_enable(&self) -> Result<(), &'static str>{
        let rcc = Rcc::new();
        rcc.apb2enr_iop_en(self.gpio_x, true)?;
        Ok(())
    }

    /// ### cr_pin_config
    /// **cnf_mode** CNFy + MODEx
    /// ####CNFy: Port configuration bits
    /// ##### Input mode
    /// - 00: Analog mode
    /// - 01: Floating input (reset state)
    /// - 10: Input with pull-up / pull-down
    /// - 11: General purpose output push-pull
    /// ##### Output mode
    /// - 00: General purpose output push-pull
    /// - 01: General purpose output Open-drain
    /// - 10: Alternate function output Push-pull
    /// - 11: Alternate function output Open-drain
    /// #### MODEx 
    /// - 00: Input mode (reset state)
    /// - 01: Output mode, max speed 10 MHz.
    /// - 10: Output mode, max speed 2 MHz.
    /// - 11: Output mode, max speed 50 MHz.
    pub fn cr_pin_config(&self, pin: u8, cnf_mode: u32) -> Result<(), &'static str> {
        match cnf_mode {
            0..=0b1111 => (), // 유효한 범위를 확장합니다.
            _ => return Err("Invalid configuration mode"),
        }
        let cr_x = self.cr_x(pin)?;
        let shift = (pin % 8) * 4;
    
        unsafe {
            let mut cr_val = cr_x.read_volatile();
            cr_val &= !(0b1111 << shift); // Clear the current configuration
            cr_val |= (cnf_mode << shift); // Set the configuration
            cr_x.write_volatile(cr_val);
        }
        Ok(())
    }
    pub fn bsrr_write(&self, port: u8) {
        unsafe {
            self.bsrr.write_volatile(0b1 << port);
        }
    }


    pub fn bsrr_reset(&self, port: u8) {
        unsafe {
            self.bsrr.write_volatile(1 << (port + 16));
        }
    }


    pub fn idr_read(&self, port: u8) -> u32 {
        unsafe { (self.idr.read_volatile() & (1 << port)) >> port }
    }

    // unsafe fn CRL(&self) -> *mut u32 {
    //     (self.base + 0x00) as *mut u32
    // }

    // unsafe fn CRH(&self) -> *mut u32 {
    //     (self.base + 0x04) as *mut u32
    // }

    // unsafe fn IDR(&self) -> *mut u32 {
    //     (self.base + 0x08) as *mut u32
    // }

    // unsafe fn BSRR(&self) -> *mut u32 {
    //     (self.base + 0x10) as *mut u32
    // }

    // pub fn bsrr_write(&self, port: u8) {
    //     unsafe {
    //         self.BSRR().write_volatile(1 << port);
    //     }
    // }

    // pub fn bsrr_reset(&self, port: u8) {
    //     unsafe {
    //         self.BSRR().write_volatile(1 << (port + 16));
    //     }
    // }
    // pub fn idr_read(&self, port: u8) -> u32 {
    //     unsafe { (self.IDR().read_volatile() & (1 << port)) >> port }
    // }

    
    // /// ## crl_port_config                                  
    // /// cnf_mode: CNFy + MODEx                              <br/>
    // /// #### CNFy: Port configuration bits (y = 0..7)       
    // /// **00**: Analog mode                                 <br/>
    // /// **01**: Floating input (reset state)                <br/>
    // /// **10**: Input with pull-up / pull-down              <br/>
    // /// **11**: General purpose output push-pull            <br/>     
    // /// #### MODEx                                          
    // /// **00**: Input mode (reset state)                    <br/>
    // /// **01**: Output mode, max speed 10 MHz.              <br/>
    // /// **10**: Output mode, max speed 2 MHz.               <br/>
    // /// **11**: Output mode, max speed 50 MHz.              <br/>
    // pub fn crl_port_config(&self, port: u8, cnf_mode: u32) {
    //     assert!(port < 8, "Port number must be between 0 and 7");
    //     unsafe {
    //         let shift = port * 4;
    //         let mut crl_val = self.CRL().read_volatile();
    //         crl_val &= !(0b1111 << shift); // Clear the current configuration
    //         crl_val |= (cnf_mode << shift); // Set the configuration
    //         self.CRL().write_volatile(crl_val);
    //     }
    // }

    

    // pub fn crh_port_config(&self, port: u8, mode: u32) {
    //     assert!(
    //         port >= 8 && port < 16,
    //         "Port number must be between 8 and 15 for CRH"
    //     );
    //     unsafe {
    //         let shift = (port - 8) * 4;
    //         let crh_reg = self.CRH();

    //         let mut cr_val = crh_reg.read_volatile();
    //         cr_val &= !(0b1111 << shift); // Clear the current configuration
    //         cr_val |= (mode << shift); // Set the mode
    //         crh_reg.write_volatile(cr_val);
    //     }
    // }
    // pub fn configure_pc2_as_analog(&self) {
    //     unsafe {
    //         let gpio_c_crl = 0x4001_1000 as *mut u32; // GPIOC_CRL 레지스터 주소
    //         let mut moder = gpio_c_crl.read_volatile();
    //         moder &= !(0b11 << (2 * 4));  // CNF2[1:0] = 00 (아날로그 모드)
    //         moder &= !(0b11 << ((2 * 4) + 2));  // MODE2[1:0] = 00 (입력 모드)
    //         gpio_c_crl.write_volatile(moder);
    //     }
    // }

}
