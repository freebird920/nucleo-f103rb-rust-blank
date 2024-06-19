#![allow(non_snake_case)]
use rtt_target::rprintln;

use crate::utils::delay::{delay_sys_clk_10us, delay_sys_clk_ms};

use super::rcc::Rcc;

pub struct I2c {
    rcc: Rcc,
    i2c_x: u8,
    cr1: *mut u32,
    cr2: *mut u32,
    ccr: *mut u32,
    trise: *mut u32,
    sr1: *mut u32,
    sr2: *mut u32,
    dr: *mut u32,
}

impl I2c {
    /// ### I2c::new
    /// **I2c_x** 1 | 2
    pub fn new(i2c_x: u8) -> Result<I2c, &'static str> {
        let base: u32 = match i2c_x {
            1 => 0x40005400,
            2 => 0x40005800,
            _ => return Err("Invalid I2C number"),
        };
        let rcc = Rcc::new();
        Ok(I2c {
            rcc: Rcc::new(),
            i2c_x: i2c_x,
            cr1: (base + 0x00) as *mut u32,
            cr2: (base + 0x04) as *mut u32,
            ccr: (base + 0x1C) as *mut u32,
            trise: (base + 0x20) as *mut u32,
            sr1: (base + 0x14) as *mut u32,
            sr2: (base + 0x18) as *mut u32,
            dr: (base + 0x10) as *mut u32,
        })
    }
    pub fn i2c_clock_enable(&self) {
        let rcc = Rcc::new();
        rcc.enable_i2c2(); // I2C1 clock enable
        rprintln!("abp1enr: {}", rcc.apb1enr_read());
    }

    pub fn cr1_pe(&self, enable: bool) {
        unsafe {
            let mut cr1_val = self.cr1.read_volatile();
            if enable {
                cr1_val |= (1 << 0); // Enable I2C
            } else {
                cr1_val &= !(1 << 0); // Disable I2C
            }
            self.cr1.write_volatile(cr1_val);
        }
    }

    pub fn cr2_freq(&self, freq: u32) -> Result<(), &'static str> {
        if (freq > 0b110010) {
            return Err("FREQ value is out of range FREQ > 50MHz NOT ALLOWED");
        };
        unsafe {
            self.rcc.cfgr_ppre1_set(8); // Set APB1 prescaler to 2 -> 32MHz
            let mut cr2_val = self.cr2.read_volatile();
            cr2_val &= !(0b111111 << 0); // Clear FREQ[5:0]
            cr2_val |= (freq << 0); // Set FREQ[5:0] to 8MHz
            self.cr2.write_volatile(cr2_val);
        }
        Ok(())
    }

    /// ### I2c::ccr_set_std
    /// **Important** CCR must be configured only when the I2C is disabled (PE = 0) <br/>
    /// **ccr** CCR[11:0] bits are used to configure the SCL clock speed <br/>
    /// **trise** TRISE[5:0] bits are used to configure the maximum rise time of SCL <br/>
    /// trise는 자동으로 설정됨.
    pub fn ccr_set_std(&self) {
        unsafe {
            let sys_clock = self.rcc.get_sys_clock();
            // rprintln!("sys_clock: {}", sys_clock);
            let ppre1_val = self.rcc.cfgr_ppre1_read();
            let apb1_clock = sys_clock / ppre1_val;
            // rprintln!("apb1_clock: {}", apb1_clock);
            // 표준 모드에서 100kHz 설정
            let i2c_clock = 100_000;
            let ccr = (apb1_clock / (i2c_clock * 2)) as u32;

            // I2C_CCR 레지스터 설정
            let mut ccr_val = self.ccr.read_volatile();
            ccr_val &= !(0b1111_1111_1111 << 0); // Clear CCR[11:0]
            ccr_val |= (ccr & 0b1111_1111_1111); // Set CCR[11:0]
            rprintln!("ccr_val: 0b{:b}", ccr_val);
            self.ccr.write_volatile(ccr_val);

            // TRISE 설정
            let trise = (apb1_clock / 1_000_000) + 1; // 표준 모드 (1000ns / T_PCLK1) + 1
            rprintln!("trise: {}", trise);
            self.trise.write_volatile(trise as u32);
        }
    }

    pub fn ccr_set_fast(&self, duty: u8) {
        unsafe {
            let sys_clock = self.rcc.get_sys_clock();
            let ppre1_val = self.rcc.cfgr_ppre1_read();
            let apb1_clock = sys_clock / ppre1_val;

            // 패스트 모드에서 400kHz 설정
            let i2c_clock = 400_000;
            let ccr = if duty == 0 {
                (apb1_clock / (i2c_clock * 3)) as u32 // DUTY 0: T_low/T_high = 2
            } else {
                (apb1_clock / (i2c_clock * 25)) as u32 // DUTY 1: T_low/T_high = 16/9
            };

            // I2C_CCR 레지스터 설정
            let mut ccr_val = self.ccr.read_volatile();
            ccr_val &= !(0b1111_1111_1111 << 0); // Clear CCR[11:0]
            ccr_val |= (ccr & 0b1111_1111_1111); // Set CCR[11:0]
            ccr_val |= 0x8000; // FS 비트 설정 (패스트 모드)
            if duty == 1 {
                ccr_val |= 0x4000; // DUTY 비트 설정
            }
            self.ccr.write_volatile(ccr_val);

            // TRISE 설정
            let trise = (apb1_clock / 3_000_000) + 1; // 패스트 모드 (300ns / T_PCLK1) + 1
            self.trise.write_volatile(trise as u32);
        }
    }

    /// ### I2c::trise_set
    /// **Important** TRISE must be configured only when the I2C is disabled (PE = 0)
    /// **trise** TRISE[5:0] bits are used to configure the maximum rise time of SCL
    pub fn trise_set(&self, trise: u32) -> Result<(), &'static str> {
        unsafe {
            if (trise > 0b11111) {
                return Err("TRISE value is out of range TRISE > 31 NOT ALLOWED");
            };
            let mut trise_val = self.trise.read_volatile();
            trise_val &= !(0b11111 << 0); // Clear TRISE[5:0]
            trise_val |= (trise << 0);
            self.trise.write_volatile(trise_val);
            Ok(())
        }
    }

    pub fn cr1_start(&self) {
        unsafe {
            let mut cr1_val = self.cr1.read_volatile();
            cr1_val |= (1 << 8); // Set the START bit (bit 8)
            self.cr1.write_volatile(cr1_val);
            let mut count = 0;
            while (self.sr1.read_volatile() & (1 << 0)) == 0 {
                count += 1;
                if count > 0b1111_1110 {
                    rprintln!("shit");
                    break;
                }
            } // Wait until the START condition is generated (SB bit is set in SR1)
        }
    }
    pub fn cr1_stop(&self) {
        unsafe {
            let mut cr1_val = self.cr1.read_volatile();
            cr1_val |= (1 << 9); // Set the STOP bit (bit 9)
            self.cr1.write_volatile(cr1_val);
        }
    }
    fn bit7_address_into_write(&self, address: u8) -> Result<u8, &'static str> {
        // 7비트 주소를 읽기 모드로 설정
        if (address) > 0b0111_1111 {
            return Err("LSB of the address must be 0 for read mode");
        }
        Ok((address << 1))
    }
    fn bit7_address_into_read(&self, address: u8) -> Result<u8, &'static str> {
        // 7비트 주소를 읽기 모드로 설정
        if (address) > 0b0111_1111 {
            return Err("LSB of the address must be 0 for read mode");
        }
        Ok((address << 1) | 0b0000_0001)
    }

    pub fn dr_send_address(&self, target_address: u8) -> Result<(), &'static str> {
        // cr1 을 설정해서 보낸다.
        self.cr1_start();
        let address_write = self.bit7_address_into_write(target_address)?;
        self.dr_set(address_write);
        let mut count: u8 = 0;
        while self.sr1_addr_read() == false {
            count += 1;
            if count > 0b1111_1110 {
                return Err("Timeout waiting for ADDR bit to be set");
            }
        }
        let _ = self.sr2_read();
        Ok(())
    }

    fn i2c_send_data(&self, data: u8) -> Result<(), &'static str> {
        // 데이터 레지스터가 비어 있는지 확인 (TXE 비트)
        while self.sr1_txe_read() == 0 {}

        // 데이터 전송
        self.dr_set(data);
        while self.sr1_txe_read() == 0 {}
        while self.sr1_btf_read() == 0 {}
        Ok(())
    }
    pub fn sr1_btf_read(&self) -> u8 {
        unsafe { (self.sr1.read_volatile() & (1 << 2)) as u8 }
    }
    pub fn sr1_txe_read(&self) -> u8 {
        unsafe { (self.sr1.read_volatile() & (1 << 7)) as u8 }
    }
    pub fn sr_btf_read(&self) -> u8 {
        unsafe { (self.sr1.read_volatile() & (1 << 2)) as u8 }
    }
    pub fn sr1_addr_read(&self) -> bool {
        unsafe { (self.sr1.read_volatile() & (1 << 1)) != 0 }
    }
    pub fn sr2_read(&self) -> u32 {
        unsafe { self.sr2.read_volatile() }
    }
    pub fn dr_set(&self, data: u8) {
        unsafe {
            self.dr.write_volatile(data.into());
        }
    }

    pub fn dr_write(&self, address: u8, data: u8) {
        self.cr1_start();
        // rprintln!("I2C start condition set");

        let address_write = address << 1;
        unsafe {
            self.dr.write_volatile(address_write.into());
            // rprintln!("I2C address written: 0x{:X}", address_write);

            // Wait until the ADDR bit is set in SR1
            let mut timeout = 1000000000; // 타임아웃 카운터 설정
            while (self.sr1.read_volatile() & (1 << 1)) == 0 {
                // rprintln!("Waiting for ADDR bit to be set");

                // Check for errors
                let sr1_val = self.sr1.read_volatile();
                if sr1_val & (1 << 8) != 0 {
                    rprintln!("I2C Bus Error");
                    break;
                }
                if sr1_val & (1 << 9) != 0 {
                    rprintln!("I2C Arbitration Lost");
                    break;
                }
                if sr1_val & (1 << 10) != 0 {
                    rprintln!("I2C Acknowledge Failure");
                    break;
                }
                if sr1_val & (1 << 11) != 0 {
                    rprintln!("I2C Overrun/Underrun");
                    break;
                }

                // 타임아웃 체크
                timeout -= 1;
                if timeout == 0 {
                    rprintln!("Timeout waiting for ADDR bit to be set");
                    break;
                }
            }

            // ADDR 비트가 설정되지 않으면 함수 종료
            if self.sr1.read_volatile() & (1 << 1) == 0 {
                self.cr1_stop();
                return;
            }

            // rprintln!("I2C address acknowledged");

            // Clear ADDR flag
            let _ = self.sr1.read_volatile();
            let _ = self.sr2.read_volatile();
            // rprintln!("I2C address cleared");

            self.dr.write_volatile(data.into());
            // rprintln!("I2C data written: 0x{:X}", data);

            // Wait until the BTF bit is set in SR1
            timeout = 1000000000; // 타임아웃 카운터 재설정
            while (self.sr1.read_volatile() & (1 << 7)) == 0 {
                rprintln!("Waiting for BTF bit to be set");

                // Check for errors
                let sr1_val = self.sr1.read_volatile();
                if sr1_val & (1 << 8) != 0 {
                    rprintln!("I2C Bus Error");
                    break;
                }
                if sr1_val & (1 << 9) != 0 {
                    rprintln!("I2C Arbitration Lost");
                    break;
                }
                if sr1_val & (1 << 10) != 0 {
                    rprintln!("I2C Acknowledge Failure");
                    break;
                }
                if sr1_val & (1 << 11) != 0 {
                    rprintln!("I2C Overrun/Underrun");
                    break;
                }

                // 타임아웃 체크
                timeout -= 1;
                if timeout == 0 {
                    rprintln!("Timeout waiting for BTF bit to be set");
                    break;
                }
            }

            // BTF 비트가 설정되지 않으면 함수 종료
            if self.sr1.read_volatile() & (1 << 7) == 0 {
                self.cr1_stop();
                return;
            }

            // rprintln!("I2C data transfer finished");

            self.cr1_stop();
            // rprintln!("I2C stop condition set");
        }
    }
}

pub struct PCF8574_LCD {
    i2c: I2c,
    address: u8,
}

impl PCF8574_LCD {
    pub fn new(i2c: I2c, address: u8) -> PCF8574_LCD {
        PCF8574_LCD {
            i2c: i2c,
            address: address,
        }
    }
    pub fn send_cmd(&self, cmd: u8) {
        let cmd_upper: u8 = (cmd & 0xF0);
        let cmd_lower: u8 = (cmd & 0x0F) << 4;

        // rprintln!("1. cmd_upper: 0x{:X}", cmd_upper);

        self.i2c.dr_write(self.address, cmd_upper | 0b1100);
        // rprintln!("2. cmd_upper | 0b1100 sent");
        delay_sys_clk_10us(5);
        // rprintln!("3. delay after cmd_upper | 0b1100");

        self.i2c.dr_write(self.address, cmd_upper | 0b1000);
        // rprintln!("4. cmd_upper | 0b1000 sent");

        self.i2c.dr_write(self.address, cmd_lower | 0b1100);
        // rprintln!("5. cmd_lower | 0b1100 sent");
        delay_sys_clk_10us(5);
        // rprintln!("6. delay after cmd_lower | 0b1100");

        self.i2c.dr_write(self.address, cmd_lower | 0b1000);
        // rprintln!("7. cmd_lower | 0b1000 sent");
    }

    pub fn lcd_initialize(&self) {
        // 초기화 절차
        delay_sys_clk_ms(500); // Wait for more than 15 ms after Vcc rises to 4.5V
        self.send_cmd(0b0011_0000); // Function set (8-bit interface)
        delay_sys_clk_10us(1); // Wait for more than 4.1 ms
        self.send_cmd(0b0011_0000); // Function set (8-bit interface)
        delay_sys_clk_10us(20); // Wait for more than 100 us
        self.send_cmd(0b0011_0000); // Function set (8-bit interface)
        delay_sys_clk_10us(20);
        self.send_cmd(0b0010_0000); // Function set (4-bit interface)
        delay_sys_clk_10us(20);

        // Function set (4-bit interface, 2-line, 5x8 dots)
        self.send_cmd(0b0010_1000); // Function set (4-bit interface, 2-line display, 5x8 dots)
        delay_sys_clk_10us(20);
        self.send_cmd(0b0010_1000); // Function set (4-bit interface, 2-line display, 5x8 dots)
        delay_sys_clk_10us(200);

        // Display on, cursor on, blink off
        self.send_cmd(0b0000_1110); // Display control: Display on, cursor on, blink off
        delay_sys_clk_10us(20);

        // Clear display
        self.send_cmd(0b0000_0001); // Clear display
        delay_sys_clk_10us(200); // This command needs a longer delay

        // Entry mode set: Increment cursor, no display shift
        self.send_cmd(0b0000_0110); // Entry mode set: Increment mode
        delay_sys_clk_10us(20);

        // self.send_cmd(0b0000_0010);
        // delay_sys_clk_10us(20);
        // Clear display
        // self.send_cmd(0b0000_0001);       // Clear display
        // delay_sys_clk_10us(200);
    }

    pub fn clear(&self) {
        self.send_cmd(0b0000_0001); // Clear display
        delay_sys_clk_10us(200); // This command needs a longer delay
    }
    pub fn display_off(&self) {
        self.send_cmd(0b0000_1100); // Display off
        delay_sys_clk_10us(20);
    }
    pub fn send_data(&self, data: u8) {
        let data_upper: u8 = (data & 0xF0);
        let data_lower: u8 = (data & 0x0F) << 4;

        // Send upper nibble
        self.i2c.dr_write(self.address, data_upper | 0b1101); // RS = 1, EN = 1
        delay_sys_clk_10us(5);
        self.i2c.dr_write(self.address, data_upper | 0b1001); // RS = 1, EN = 0

        // Send lower nibble
        self.i2c.dr_write(self.address, data_lower | 0b1101); // RS = 1, EN = 1
        delay_sys_clk_10us(5);
        self.i2c.dr_write(self.address, data_lower | 0b1001); // RS = 1, EN = 0
    }
    pub fn print(&self, str: &str) {
        for c in str.bytes() {
            self.send_data(c);
        }
    }
    pub fn print_number(&self, number: u32) {
        let mut num = number;
        let mut buffer = [0u8; 10];
        let mut i = 0;

        if num == 0 {
            self.send_data('0' as u8);
            return;
        }

        while num > 0 {
            buffer[i] = (num % 10) as u8 + '0' as u8;
            num /= 10;
            i += 1;
        }

        while i > 0 {
            i -= 1;
            self.send_data(buffer[i]);
        }
    }
    pub fn set_cursor(&self, row: u8, col: u8) {
        let mut address = match row {
            0 => 0x80 + col,
            1 => 0xC0 + col,
            _ => 0x80 + col, // 기본적으로 첫 번째 행을 사용
        };
        self.send_cmd(address);
    }
}
