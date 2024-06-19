use crate::peripherals::i2c::I2c;

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
    pub fn send_address(&self)->Result<(),&'static str> {
        self.i2c.dr_send_address(self.address,)?;
        Ok(())
    }
    pub fn send_cmd(&self, cmd: u8) -> Result<(), &'static str> {
        self.i2c.dr_send_data(self.address , cmd)?;
        Ok(())
    }
}
