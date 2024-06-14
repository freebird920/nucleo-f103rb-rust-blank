#![allow(non_snake_case)]

pub enum BASE_ADC {
    ADC_1 = 0x4001_2800,
    ADC_2 = 0x4001_2400,
}

pub struct ADC {
    base: u32, // Storing the base address directly as u32
    sr: *mut u32,
    cr1: *mut u32,
    cr2: *mut u32,
}

impl ADC {
    pub fn new(base: BASE_ADC) -> ADC {
        let base_addr = base as u32; // Cast BASE_ADC to u32 to get the base address
        ADC {
            base: base_addr,
            sr: (base_addr + 0x00) as *mut u32,
            cr1: (base_addr + 0x04) as *mut u32,
            cr2: (base_addr + 0x08) as *mut u32,
        }
    }




}
