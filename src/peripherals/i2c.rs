const I2C2_BASE : u32 = 0x4000_5800;

const I2C2_CR1:     *mut u32    = (I2C2_BASE + 0x00)        as *mut u32;
const I2C2_CR2:     *mut u32    = (I2C2_BASE + 0x04)        as *mut u32;
const I2C2_CCR:     *mut u32    = (I2C2_BASE + 0x1C)        as *mut u32;
const I2C2_TRISE:   *mut u32    = (I2C2_BASE + 0x20)        as *mut u32;
const I2C2_SR1 :    *mut u32    = (I2C2_BASE + 0x14)        as *mut u32;
const I2C2_SR2 :    *mut u32    = (I2C2_BASE + 0x18)        as *mut u32;
const I2C2_DR:      *mut u32    = (I2C2_BASE + 0x10)        as *mut u32;
pub unsafe fn init() {
        // Set I2C2
        I2C2_CR1.write_volatile(I2C2_CR1.read_volatile() & !(1 << 0)); // Disable I2C2

        let mut i2c2_cr2_val = I2C2_CR2.read_volatile();
        i2c2_cr2_val &= !(0b111111 << 0); // Clear FREQ[5:0]
        i2c2_cr2_val |= (8 << 0); // Set FREQ[5:0] to 8MHz
        I2C2_CR2.write_volatile(i2c2_cr2_val);

        let mut i2c2_ccr_val = I2C2_CCR.read_volatile();
        i2c2_ccr_val &= !(0b1111_1111_1111 << 0); // Clear CCR[11:0]
        i2c2_ccr_val |= (40 << 0); // Set CCR[11:0] to 40 (Standard mode, 100 kHz)
        I2C2_CCR.write_volatile(i2c2_ccr_val);

        let mut i2c2_trise_val = I2C2_TRISE.read_volatile();
        i2c2_trise_val &= !(0b11111 << 0); // Clear TRISE[5:0]
        i2c2_trise_val |= (9 << 0); // Set TRISE[5:0] to 9
        I2C2_TRISE.write_volatile(i2c2_trise_val);

        I2C2_CR1.write_volatile(I2C2_CR1.read_volatile() | (1 << 0)); // Enable I2C2
        // End I2C2
}

pub unsafe fn i2c2_start() {
    // Read the current value of I2C2_CR1
    let mut i2c2_cr1_val = I2C2_CR1.read_volatile();
    // Set the START bit (bit 8)
    i2c2_cr1_val |= (1 << 8);
    // Write the modified value back to I2C2_CR1
    I2C2_CR1.write_volatile(i2c2_cr1_val);
    // Wait until the START condition is generated (SB bit is set in SR1)
    while (I2C2_SR1.read_volatile() & (1 << 0)) == 0 {}
}

pub unsafe fn i2c2_stop(){
    let mut i2c2_cr1_val = I2C2_CR1.read_volatile();
    i2c2_cr1_val |= (1 << 9);
    I2C2_CR1.write_volatile(i2c2_cr1_val);
}

pub unsafe fn i2c2_write(address : u8, data : u8){
    i2c2_start();
    let address_write = address << 1;
    // Write the address of the slave device to I2C2_DR
    I2C2_DR.write_volatile(address_write.into());
    // Wait until the ADDR bit is set in SR1
    while (I2C2_SR1.read_volatile() & (1 << 1)) == 0 {}
    // Read the SR2 register to clear the ADDR bit
    let _ = I2C2_SR1.read_volatile();
    let _ = I2C2_SR2.read_volatile(); // ADDR Clear
    // Write the data to I2C2_DR
    I2C2_DR.write_volatile(data.into());
    // Wait until the BTF bit is set in SR1
    while (I2C2_SR1.read_volatile() & (1 << 7)) == 0 {}
    i2c2_stop();
    
}