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

    pub unsafe fn port_config(&self, port: u8, cnf: u32, mode: u32) {
        // Check if the port number is valid
        assert!(port < 16, "Port number must be between 0 and 15");

        let shift = (port % 8) * 4;
        let cr_reg = if port < 8 { self.CRL() } else { self.CRH() };

        let mut cr_val = cr_reg.read_volatile();
        cr_val &= !(0b1111 << shift); // Clear the current configuration
        cr_val |= (cnf << shift); // Set the configuration
        cr_val |= (mode << (shift + 2)); // Set the mode
        cr_reg.write_volatile(cr_val);
    }
}

const GPIOA_BASE: u32 = 0x4001_0800;
const GPIOB_BASE: u32 = 0x4001_0C00;
const GPIOC_BASE: u32 = 0x4001_1000;

const GPIOA_CRL: *mut u32 = (GPIOA_BASE + 0x00) as *mut u32;
const GPIOA_BSRR: *mut u32 = (GPIOA_BASE + 0x10) as *mut u32;
const GPIOB_CRH: *mut u32 = (GPIOB_BASE + 0x04) as *mut u32;
const GPIOC_IDR: *mut u32 = (GPIOC_BASE + 0x08) as *mut u32;
const GPIOC_CRH: *mut u32 = (GPIOC_BASE + 0x04) as *mut u32;

pub unsafe fn init() {
    // Set GPIOA pin 5 to output push-pull
    let mut gpioa_crl_val = GPIOA_CRL.read_volatile();
    gpioa_crl_val &= !(0b1111 << 20); // Clear CNF5[1:0] and MODE5[1:0]
    gpioa_crl_val |= (0b0011 << 20); // Set MODE5[1:0] to 01 (Output mode, max speed 10 MHz)
    GPIOA_CRL.write_volatile(gpioa_crl_val);

    // Set GPIOB pin 10 I2C2 SCL, pin 11 I2C2 SDA to alternate function open-drain
    let mut gpiob_crh_val = GPIOB_CRH.read_volatile();
    gpiob_crh_val &= !(0b1111 << 8); // Clear CNF10[1:0] and MODE10[1:0]
    gpiob_crh_val &= !(0b1111 << 12); // Clear CNF11[1:0] and MODE11[1:0]
    gpiob_crh_val |= (0b1011 << 8); // Set CNF10[1:0] to 10 (Alternate function output open-drain)
    gpiob_crh_val |= (0b1011 << 12); // Set CNF11[1:0] to 10 (Alternate function output open-drain)
    GPIOB_CRH.write_volatile(gpiob_crh_val);

    // Set GPIOC pin 13 to input floating (reset state)
    let mut gpioc_crh_val = GPIOC_CRH.read_volatile();
    gpioc_crh_val &= !(0b1111 << 20); // Clear CNF13[1:0] and MODE13[1:0]
    gpioc_crh_val |= (0b0100 << 20); // Set CNF13[1:0] to 01 (Floating input)
    GPIOC_CRH.write_volatile(gpioc_crh_val);
}

pub unsafe fn read_button() -> u32 {
    GPIOC_IDR.read_volatile() & (1 << 13)
}

pub fn led_on() {
    unsafe {
        GPIOA_BSRR.write_volatile(1 << 5);
    }
}

pub fn led_off() {
    unsafe {
        GPIOA_BSRR.write_volatile(1 << (5 + 16));
    }
}
