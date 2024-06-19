use crate::peripherals::i2c::I2c;

pub struct Pcf8574 {
    i2c: I2c,
    address: u8,
}

impl Pcf8574 {
    pub fn new(i2c: I2c, a2: u8, a1:u8, a0:u8) -> Self {
        let mut address = 0b0100_0000;
        address |= a2 << 2;
        address |= a1 << 1;
        address |= a0;
        Pcf8574 { 
            i2c, 
            address: address,
        }
    }
    pub fn write (){
        
    }
    
}