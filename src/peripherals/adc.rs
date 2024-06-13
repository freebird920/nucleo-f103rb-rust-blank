#![allow(non_snake_case)]

pub enum BASE_ADC{
    ADC_1 = 0x4001_2800,
    ADC_2 = 0x4001_2400,

}

// const ADC_1_BASE: u32  = 0x4001_2800;
// const ADC_2_BASE: u32  = 0x4001_2400;
pub struct ADC{
    base: u32,
}

impl ADC {


    pub fn new (base: BASE_ADC) -> ADC {
        ADC {
            base: base as u32,
        }
    }
    /// ADC_SR ADC status register
    fn SR(&self) -> *mut u32 {
        (self.base + 0x00) as *mut u32
    }

    fn CR1(&self) -> *mut u32 {
        (self.base + 0x04) as *mut u32
    }

    fn CR2(&self) -> *mut u32 {
        (self.base + 0x08) as *mut u32
    }



}
