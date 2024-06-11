use crate::utils::delay::delay_sys_clk_ms;

enum BASE_I2C {
    I2C2 = 0x4000_5800,
}
pub struct I2C {
    base: u32,
}

impl I2C {
    pub fn new(base: BASE_I2C) -> I2C {
        I2C { base: base as u32 }
    }
    fn CR1(&self) -> *mut u32 {
        (self.base + 0x00) as *mut u32
    }
    fn CR2(&self) -> *mut u32 {
        (self.base + 0x04) as *mut u32
    }
    fn CCR(&self) -> *mut u32 {
        (self.base + 0x1C) as *mut u32
    }
    fn TRISE(&self) -> *mut u32 {
        (self.base + 0x20) as *mut u32
    }
    fn SR1(&self) -> *mut u32 {
        (self.base + 0x14) as *mut u32
    }
    fn SR2(&self) -> *mut u32 {
        (self.base + 0x18) as *mut u32
    }
    fn DR(&self) -> *mut u32 {
        (self.base + 0x10) as *mut u32
    }
    pub fn cr1_pe(&self, enable: bool) {
        unsafe {
            let mut cr1_val = self.CR1().read_volatile();
            if enable {
                cr1_val |= (1 << 0); // Enable I2C
            } else {
                cr1_val &= !(1 << 0); // Disable I2C
            }
            self.CR1().write_volatile(cr1_val);
        }
    }
    pub fn cr2_freq(&self, freq: u32) {
        if (freq > 0b110010) {
            panic!("FREQ value is out of range FREQ > 50MHz NOT ALLOWED");
        };
        unsafe {
            let mut cr2_val = self.CR2().read_volatile();
            cr2_val &= !(0b111111 << 0); // Clear FREQ[5:0]
            cr2_val |= (freq << 0); // Set FREQ[5:0] to 8MHz
            self.CR2().write_volatile(cr2_val);
        }
    }
    pub fn ccr_set(&self, ccr: u32) {
        unsafe {
            let mut ccr_val = self.CCR().read_volatile();
            ccr_val &= !(0b1111_1111_1111 << 0); // Clear CCR[11:0]
            ccr_val |= (ccr << 0);
            self.CCR().write_volatile(ccr_val);
        }
    }
    pub fn trise_set(&self, trise: u32) {
        unsafe {
            let mut trise_val = self.TRISE().read_volatile();
            trise_val &= !(0b11111 << 0); // Clear TRISE[5:0]
            trise_val |= (trise << 0);
            self.TRISE().write_volatile(trise_val);
        }
    }
    pub fn init(&self) {
        self.cr1_pe(false); // Disable I2C
        self.cr2_freq(8); // Set FREQ[5:0] to 8MHz
        self.ccr_set(40); // Set CCR[11:0] to 40 (Standard mode, 100 kHz)
        self.trise_set(9); // Set TRISE[5:0] to 9
        self.cr1_pe(true); // Enable I2C
    }
    pub fn cr1_start(&self) {
        unsafe {
            let mut cr1_val = self.CR1().read_volatile();
            cr1_val |= (1 << 8); // Set the START bit (bit 8)
            self.CR1().write_volatile(cr1_val);
            while (self.SR1().read_volatile() & (1 << 0)) == 0 {} // Wait until the START condition is generated (SB bit is set in SR1)
        }
    }
    pub fn cr1_stop(&self) {
        unsafe {
            let mut cr1_val = self.CR1().read_volatile();
            cr1_val |= (1 << 9); // Set the STOP bit (bit 9)
            self.CR1().write_volatile(cr1_val);
        }
    }
    pub fn dr_write(&self, address: u8, data: u8) {
        self.cr1_start();
        let address_write = address << 1;
        unsafe {
            self.DR().write_volatile(address_write.into());
            while (self.SR1().read_volatile() & (1 << 1)) == 0 {} // Wait until the ADDR bit is set in SR1
            let _ = self.SR1().read_volatile(); // Read the SR1 register to clear the ADDR bit
            let _ = self.SR2().read_volatile(); // ADDR Clear
            self.DR().write_volatile(data.into());
            while (self.SR1().read_volatile() & (1 << 7)) == 0 {} // Wait until the BTF bit is set in SR1
            self.cr1_stop();
        }
    }
}

struct PCF8574_LCD {
    i2c: I2C,
    address: u8,
}

impl PCF8574_LCD {
    pub fn new(i2c: I2C, address: u8) -> PCF8574_LCD {
        PCF8574_LCD {
            i2c: i2c,
            address: address,
        }
    }
    pub fn send_cmd(&self, cmd: u8) {
        let address_write: u8 = self.address << 1;
        let cmd_upper: u8 = (cmd & 0xF0);
        let cmd_lower: u8 = (cmd & 0x0F) << 4;

        self.i2c.dr_write(address_write, cmd_upper | 0b1100);
        delay_sys_clk_ms(1);
        self.i2c.dr_write(address_write, cmd_upper | 0b1000);

        self.i2c.dr_write(address_write, cmd_lower | 0b1100);
        delay_sys_clk_ms(1);
        self.i2c.dr_write(address_write, cmd_lower | 0b1000);
    }
    pub fn lcd_init(&self) {
        unsafe {
            let address_write: u8 = self.address << 1;
            self.send_cmd(0x30);
            delay_sys_clk_ms(1);
            self.send_cmd(0x30);
            delay_sys_clk_ms(1);
            self.send_cmd(0x30);
            delay_sys_clk_ms(1);
            self.send_cmd(0x20);
            delay_sys_clk_ms(1);
        }
    }
}
