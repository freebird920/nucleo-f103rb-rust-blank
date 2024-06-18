#![allow(non_snake_case)]
use rtt_target::rprintln;

use crate::utils::delay::{delay_sys_clk_ms, delay_sys_clk_10us};

use super::rcc::Rcc;

pub struct I2c {
    sys_clock: u32,
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
    pub fn new(I2c_x : u8) -> Result<I2c, &'static str> {
        let base: u32 = match I2c_x {
            1 => 0x40005400,
            2 => 0x40005800,
            _ => return Err("Invalid I2C number"),
        };
        let rcc = Rcc::new();
        Ok(    
        I2c { 
            sys_clock: rcc.get_sys_clock(),
            cr1: (base + 0x00) as *mut u32,
            cr2: (base + 0x04) as *mut u32,
            ccr: (base + 0x1C) as *mut u32,
            trise: (base + 0x20) as *mut u32,
            sr1: (base + 0x14) as *mut u32,
            sr2: (base + 0x18) as *mut u32,
            dr: (base + 0x10) as *mut u32,
         }

         )
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
    pub fn cr2_freq(&self, freq: u32) {
        if (freq > 0b110010) {
            panic!("FREQ value is out of range FREQ > 50MHz NOT ALLOWED");
        };
        unsafe {
            let mut cr2_val = self.cr2.read_volatile();
            cr2_val &= !(0b111111 << 0); // Clear FREQ[5:0]
            cr2_val |= (freq << 0); // Set FREQ[5:0] to 8MHz
            self.cr2.write_volatile(cr2_val);
        }
    }
    pub fn ccr_set(&self, ccr: u32) {
        unsafe {
            let mut ccr_val = self.ccr.read_volatile();
            ccr_val &= !(0b1111_1111_1111 << 0); // Clear CCR[11:0]
            ccr_val |= (ccr << 0);
            self.ccr.write_volatile(ccr_val);
        }
    }
    pub fn trise_set(&self, trise: u32) {
        unsafe {
            let mut trise_val = self.trise.read_volatile();
            trise_val &= !(0b11111 << 0); // Clear TRISE[5:0]
            trise_val |= (trise << 0);
            self.trise.write_volatile(trise_val);
        }
    }
    pub fn init(&self) {
        self.cr1_pe(false); // Disable I2C
        self.cr2_freq(8); // Set FREQ[5:0] to 8MHz
        self.ccr_set(40); // Set CCR[11:0] to 40 (Standard mode, 100 kHz)
        self.trise_set(9); // Set TRISE[5:0] to 9
        self.cr1_pe(true); // Enable I2C

        delay_sys_clk_ms(50);
    }
    pub fn cr1_start(&self) {
        unsafe {
            let mut cr1_val = self.cr1.read_volatile();
            cr1_val |= (1 << 8); // Set the START bit (bit 8)
            self.cr1.write_volatile(cr1_val);
            while (self.sr1.read_volatile() & (1 << 0)) == 0 {} // Wait until the START condition is generated (SB bit is set in SR1)
        }
    }
    pub fn cr1_stop(&self) {
        unsafe {
            let mut cr1_val = self.cr1.read_volatile();
            cr1_val |= (1 << 9); // Set the STOP bit (bit 9)
            self.cr1.write_volatile(cr1_val);
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
        if  self.sr1.read_volatile() & (1 << 7) == 0 {
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
        delay_sys_clk_ms(500);             // Wait for more than 15 ms after Vcc rises to 4.5V
        self.send_cmd(0b0011_0000);       // Function set (8-bit interface)
        delay_sys_clk_10us(1);            // Wait for more than 4.1 ms
        self.send_cmd(0b0011_0000);       // Function set (8-bit interface)
        delay_sys_clk_10us(20);           // Wait for more than 100 us
        self.send_cmd(0b0011_0000);       // Function set (8-bit interface)
        delay_sys_clk_10us(20);
        self.send_cmd(0b0010_0000);       // Function set (4-bit interface)
        delay_sys_clk_10us(20);
    
        // Function set (4-bit interface, 2-line, 5x8 dots)
        self.send_cmd(0b0010_1000);       // Function set (4-bit interface, 2-line display, 5x8 dots)
        delay_sys_clk_10us(20);
        self.send_cmd(0b0010_1000);       // Function set (4-bit interface, 2-line display, 5x8 dots)
        delay_sys_clk_10us(200);

        // Display on, cursor on, blink off
        self.send_cmd(0b0000_1110);       // Display control: Display on, cursor on, blink off
        delay_sys_clk_10us(20);
    
        // Clear display
        self.send_cmd(0b0000_0001);       // Clear display
        delay_sys_clk_10us(200);         // This command needs a longer delay
    
        // Entry mode set: Increment cursor, no display shift
        self.send_cmd(0b0000_0110);       // Entry mode set: Increment mode
        delay_sys_clk_10us(20);

        // self.send_cmd(0b0000_0010);
        // delay_sys_clk_10us(20);
                // Clear display
                // self.send_cmd(0b0000_0001);       // Clear display
                // delay_sys_clk_10us(200);  
    }

    pub fn clear(&self) {
        self.send_cmd(0b0000_0001);       // Clear display
        delay_sys_clk_10us(200);         // This command needs a longer delay
    }
    pub fn display_off(&self) {
        self.send_cmd(0b0000_1100);       // Display off
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
