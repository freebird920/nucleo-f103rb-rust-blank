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

    /// ## CR1_ADON -  A/D Converter ON / OFF
    /// - ADC 제어 레지스터에서 ADON 비트 이외의 다른 비트가 동시에 변경되면 변환이 트리거되지 않음. <br/>
    /// - 만약 ADON 비트 이외의 다른 비트를 변경할 때 ADON 비트를 함께 변경하려고 하면, 변환이 트리거되지 않습니다. <br/>
    /// - ADC 설정 시 다른 비트와 ADON 비트를 동시에 변경하지 않도록 주의해야 합니다. 먼저 다른 비트를 설정한 후, ADON 비트를 변경하여 ADC를 켜거나 끄는 것이 좋습니다. <br/>
    pub fn cr2_adon (&self, enable: bool){
        unsafe {
            let mut adc_cr2_val = self.cr2.read_volatile();
            if enable {
                adc_cr2_val |= (1 << 0); // Enable ADC
            } else {
                adc_cr2_val &= !(1 << 0); // Disable ADC
            }
            self.cr2.write_volatile(adc_cr2_val);
        }

    }




}
