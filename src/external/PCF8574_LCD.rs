mod peripherals;
mod utils;
pub unsafe fn send_cmd ( address: u8, cmd: u8 ){
    let address_write:u8 = address << 1;
    let cmd_upper:u8 = (cmd & 0xF0);
    let cmd_lower:u8 = (cmd & 0x0F) << 4;
    

    peripherals::i2c::i2c2_write(address, cmd_upper | 0b1100);
    utils::delay::delay_us(50);
    peripherals::i2c::i2c2_write(address, cmd_upper | 0b1000);



}