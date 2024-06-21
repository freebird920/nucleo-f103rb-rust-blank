pub struct Gpio {
    // base: u32,
    gpio_x : u8,
    crl: *mut u32 ,
    crh: *mut u32,
    bsrr: *mut u32,
    idr: *mut u32,
}

impl Gpio {
    /// ### Gpio::new
    /// **gpio_x** 
    /// - 0: GPIOA
    /// - 1: GPIOB
    /// - 2: GPIOC
    pub fn new(gpio_x: u8) -> Result<Gpio, &'static str> {

        let base: Result<u32, &'static str> = match gpio_x {
            0 => Ok(0x4001_0800),
            1 => Ok(0x4001_0C00),
            2 => Ok(0x4001_1000),
            _ => Err(("Invalid GPIO port")),
        };
        let base_value = match base {
            Ok(value) => value,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(Gpio {
            // base: base_value,
            gpio_x: gpio_x,
            crl: (base_value + 0x00) as *mut u32,
            crh: (base_value + 0x04) as *mut u32,
            bsrr: (base_value + 0x10) as *mut u32,
            idr: (base_value + 0x08) as *mut u32,
        })
    }
    pub fn get_gpio_x(&self) -> u8 {
        self.gpio_x
    }
}

impl Gpio {

}

