use cortex_m::asm::delay;

use crate::peripherals::{i2c::I2c, rcc::Rcc};

pub struct Hd44780<'life_hd44780>{
    address: u8,
    i2c: &'life_hd44780 I2c,
}


impl<'a>  Hd44780<'a>  {
    /// # 이거
    /// ### RS
    /// - 0: 명령어 레지스터 선택
    /// - 1: 데이터 레지스터 선택
    /// ### RW
    /// - 0: 쓰기
    /// - 1: 읽기
    /// ### E
    /// 데이터를 쓰거나 읽기 위해 사용. 데이터가 쓰일 때 High -> Low로 전환합니다.
    /// DB0  ~ DB7 데이터 버스  
    /// 
    /// ### 0bABCD_EFGH
    /// - E = 항상 1
    /// - F = E: 1 (Enable 신호, LCD에 데이터를 전송하는 동안 활성화)
    /// - G = RW: 0 (쓰기) 1 (읽기)
    /// - H = RS: 0 (명령어 레지스터 선택) 1 (데이터 레지스터 선택)
    /// - A~D: 명령어 또는 데이터
    pub fn new(address:u8, i2c: &'a I2c) -> Self {
        Hd44780 {
            address: address,
            i2c: i2c,
        }
    }

    fn delay_us(&self, us: u32) {
        let rcc = Rcc::new();
        let sys_clock = rcc.get_sys_clock();
        let clock_per_us = sys_clock / 1_000_000;
        delay(clock_per_us * us);
    }
    fn delay_ms(&self, ms: u32) {
        let rcc = Rcc::new();
        let sys_clock = rcc.get_sys_clock();
        let clock_per_ms = sys_clock / 1_000;
        delay(clock_per_ms * ms);
    }
    pub fn send_address(&self)->Result<(),&'static str> {
        self.i2c.dr_send_address(self.address,)?;
        Ok(())
    }



    pub fn send_cmd(&self, cmd: u8) {
        let cmd_upper: u8 = (cmd & 0xF0);
        let cmd_lower: u8 = (cmd & 0x0F) << 4;
    
        // rprintln!("1. cmd_upper: 0x{:X}", cmd_upper);
    
        self.i2c.dr_write(self.address, cmd_upper | 0b1100);
        self.delay_us(50);


        self.i2c.dr_write(self.address, cmd_upper | 0b1000);
        self.delay_us(50);
    
        self.i2c.dr_write(self.address, cmd_lower | 0b1100);
        self.delay_us(50);

        self.i2c.dr_write(self.address, cmd_lower | 0b1000);
        self.delay_us(50);

    }
    /// ## send_nibble
    /// 4비트 모드로 데이터를 전송합니다. <br/>
    /// - e : Enable 신호 0: Disable 1: Enable
    /// - rw: Read/Write 신호 0: 쓰기 1: 읽기
    /// - rs: Register Select 신호 0: 명령어 레지스터 1: 데이터 레지스터
    pub fn send_nibble(&self, data: u8,  rw:u8, rs: u8) -> Result<(), &'static str> {
        if(data > 0b1111) {
            return Err("data must be 4bit");
        }
        if(rw > 1) {
            return Err("rw must be 0 or 1");
        }
        if(rs > 1) {
            return Err("rs must be 0 or 1");
        }
        let mut select_code = 0b1111 & (0b1 << 3 | 0b1<<2 | rw << 1 | rs << 0);
        let mut nibble = (data << 4) | select_code;
        self.i2c.dr_send_data(self.address, nibble)?;
        
        delay(1000 * 50);

        select_code = select_code & 0b1011;
        nibble = (data << 4) | select_code;

        self.i2c.dr_send_data(self.address, nibble)?;
        delay(1000 * 50);
        Ok(())
    }
    pub fn write_4bit(&self, data: u8) -> Result<(), &'static str> {
        
        Ok(())
    }
    pub fn send_data(&self, data: u8) {
        let data_upper: u8 = (data & 0xF0);
        let data_lower: u8 = (data & 0x0F) << 4;

        // Send upper nibble
        self.i2c.dr_write(self.address, data_upper | 0b1101); // RS = 1, EN = 1
        self.delay_us(100);
        self.i2c.dr_write(self.address, data_upper | 0b1001); // RS = 1, EN = 0
        self.delay_us(100);

        // Send lower nibble
        self.i2c.dr_write(self.address, data_lower | 0b1101); // RS = 1, EN = 1
        self.delay_us(100);

        self.i2c.dr_write(self.address, data_lower | 0b1001); // RS = 1, EN = 0
        self.delay_us(100);

    }
    pub fn print(&self, str: &str) {
        for c in str.bytes() {
            self.send_data(c);
        }
    }

    pub fn check_busy_flag(&self) -> Result<bool, &'static str> {
        // PCF8574의 핀을 통해 읽기 모드로 설정
        let busy_flag_command = 0xF0 | 0b0001_0010; // RW=1, RS=0
        self.i2c.write_byte(self.address, busy_flag_command)?;

        // PCF8574에서 데이터 읽기
        let data = self.i2c.read_byte(self.address)?;

        // Busy Flag는 DB7에 위치
        Ok((data & 0b1000_0000) != 0)
    }
    pub fn wait_until_not_busy(&self) -> Result<(), &'static str> {
        while self.check_busy_flag()? {
            // 일정 시간 대기
            self.delay_us(5);
        }
        Ok(())
    }

}
