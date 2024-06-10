mod peripherals;
mod utils;
pub unsafe fn send_cmd ( address: u8, cmd: u8 ){
    let address_write:u8 = address << 1;
    let cmd_upper:u8 = (cmd & 0xF0);
    let cmd_lower:u8 = (cmd & 0x0F) << 4;
    

    peripherals::i2c::i2c2_write(address, cmd_upper | 0b1100);
    utils::delay::delay_tik(10);
    peripherals::i2c::i2c2_write(address, cmd_upper | 0b1000);

    peripherals::i2c::i2c2_write(address, cmd_lower | 0b1100);
    utils::delay::delay_tik(10);
    peripherals::i2c::i2c2_write(address, cmd_lower | 0b1000);
}

pub unsafe fn lcd_init(address: u8){
    let address_write:u8 = address << 1;
    peripherals::i2c::i2c2_write(address, 0x30);
    utils::delay::delay_ms(5);
    peripherals::i2c::i2c2_write(address, 0x30);
    utils::delay::delay_ms(5);
    peripherals::i2c::i2c2_write(address, 0x30);
    utils::delay::delay_ms(5);
    peripherals::i2c::i2c2_write(address, 0x20);
    utils::delay::delay_ms(50);
}