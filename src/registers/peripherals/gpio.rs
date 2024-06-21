use core::f32::consts::E;

pub struct Gpio {
    // base: u32,
    gpio_x: u8,
    crl: *mut u32,
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
    pub fn get_gpio_x(&self) -> u8 {
        self.gpio_x
    }
}

impl Gpio {
    fn read_reg(&self, reg: *mut u32) -> u32 {
        unsafe { core::ptr::read_volatile(reg) }
    }

    fn write_reg(&self, reg: *mut u32, val: u32) {
        unsafe { core::ptr::write_volatile(reg, val) }
    }
}

impl Gpio {
    // CRL 0x00 , CRH 0x04
    /// ## GPIOx_CRx [0x00 ~ 0x04]
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
    pub fn cr_set(&self, pin: u8, cnf_mode: u32) -> Result<(), &'static str> {
        let cr_x = self.cr_x(pin)?;
        let shift = (pin % 8) * 4;
        let mut cr_val = self.read_reg(cr_x);
        cr_val &= !(0b1111 << shift); // Clear the current configuration
        cr_val |= (cnf_mode << shift); // Set the configuration
        self.write_reg(cr_x, cr_val);
        Ok(())
    }

    /// ### cr_x
    /// **pin** pin은 0 ~ 15까지의 값을 가질 수 있습니다.
    pub fn cr_x(&self, pin: u8) -> Result<*mut u32, &'static str> {
        match pin {
            0..=7 => Ok(self.crl),
            8..=15 => Ok(self.crh),
            _ => Err("Invalid pin number"),
        }
    }
}

impl Gpio {
    /// ## GPIOx_BSRR [0x10]
    /// ### bsrr_set [31:16] Reset, [15:0] Set
    /// - **pin** pin은 0 ~ 15까지의 값을 가질 수 있습니다.
    /// - **value** 0: Reset, 1: Set
    
    /// ### bsrr_br [31:16] Reset
    pub fn bsrr_br(&self, pin: u8) -> Result<(),&'static str> {
        if pin > 15 {return Err("Invalid pin number")};
        let val = 1 << (pin + 16);
        self.write_reg(self.bsrr, val);
        Ok(())
    }

    /// ### bsrr_bs [15:0] Set
    pub fn bsrr_bs(&self, pin: u8) -> Result<(),&'static str> {
        if pin > 15 {return Err("Invalid pin number")};
        let val = 1 << pin;
        self.write_reg(self.bsrr, val);
        Ok(())
    }
}
