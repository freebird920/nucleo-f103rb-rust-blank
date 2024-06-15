#![allow(non_snake_case)]

pub enum BaseAdc {
    Adc1 = 0x4001_2800,
    Adc2 = 0x4001_2400,
}

pub struct Adc {
    base: u32, // Storing the base address directly as u32
    sr:     *mut u32,
    cr1:    *mut u32,
    cr2:    *mut u32,
    seq1:   *mut u32,
    seq2:   *mut u32,
    seq3:   *mut u32,
    dr:     *mut u32,
}

impl Adc {
    pub fn new(base: BaseAdc) -> Adc {
        let base_addr = base as u32; // Cast BASE_ADC to u32 to get the base address
        Adc {
            base: base_addr,
            sr:     (base_addr + 0x00) as *mut u32,
            cr1:    (base_addr + 0x04) as *mut u32,
            cr2:    (base_addr + 0x08) as *mut u32,
            seq1:   (base_addr + 0x0C) as *mut u32,
            seq2:   (base_addr + 0x18) as *mut u32,
            seq3:   (base_addr + 0x34) as *mut u32,
            dr:     (base_addr + 0x4C) as *mut u32,
        }
    }

    /// ### CR1_ADON -  A/D Converter ON / OFF
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


    /// ### CR2_CAL - Calibration
    /// - ADC 제어 레지스터에서 CAL 비트는 ADC의 자체 보정을 시작하거나 보정 중인지를 나타냅니다. <br/>
    /// - CAL 비트는 ADCAL 비트가 0으로 설정되면 자동으로 0으로 클리어됩니다. <br/>
    pub fn cr2_cal (&self) {
        unsafe {
            let mut adc_cr2_val = self.cr2.read_volatile();
            adc_cr2_val |= (1 << 2); // Start calibration
            self.cr2.write_volatile(adc_cr2_val);
            while (self.cr2.read_volatile() & (1 << 2)) != 0 {
                // Wait for calibration to complete
            }
        }
    }
    pub fn cr2_cont (&self, enable: bool){
        unsafe {
            let mut adc_cr2_val = self.cr2.read_volatile();
            if enable {
                adc_cr2_val |= (1 << 1); // Continuous conversion
            } else {
                adc_cr2_val &= !(1 << 1); // Single conversion
            }
            self.cr2.write_volatile(adc_cr2_val);
        }
    }

    /// ### CR2_EXTSEL - External Event Select for Regular Group
    /// - ADC 제어 레지스터에서 EXTSEL[2:0] 비트는 외부 이벤트를 선택하여 변환을 시작할 수 있습니다. <br/>
    /// #### ADC1 ADC2
    /// - **000**: Timer 1 CC1 event
    /// - **001**: Timer 1 CC2 event
    /// - **010**: Timer 1 CC3 event
    /// - **011**: Timer 2 CC2 event
    /// - **100**: Timer 3 TRGO event
    /// - **101**: Timer 4 CC4 event
    /// - **110**: EXTI Line 11
    /// - **111**: SWSTART
    pub fn cr2_extsel (&self, extsel: u32){
        unsafe {
            let mut adc_cr2_val = self.cr2.read_volatile();
            adc_cr2_val &= !(0b111 << 17); // Clear the bits
            adc_cr2_val |= (extsel) << 17; // Set the bits
            self.cr2.write_volatile(adc_cr2_val);
        }
    }
    pub fn cr2_swstart (&self , enable: bool){
        unsafe {
            let mut adc_cr2_val = self.cr2.read_volatile();
            if enable {
                adc_cr2_val |= (1 << 22); // Start conversion
            } else {
                adc_cr2_val &= !(1 << 22); // Stop conversion
            }
            self.cr2.write_volatile(adc_cr2_val);
        }
    }
    pub fn sqr3_sq(&self, seq: u8, channel: u32){
        unsafe {
            let mut adc_sqr3_val = self.seq3.read_volatile();
            let shift = (seq - 1) * 5;
            adc_sqr3_val &= !(0b11111 << shift); // Clear the bits
            adc_sqr3_val |= (channel) << shift; // Set the bits
            self.seq3.write_volatile(adc_sqr3_val);
        }
    }
    pub fn sqr_sq(&self, seq: u8, channel: u32){
        unsafe {
            let this_sqr = match seq {
                1..=6 => self.seq3,
                7..=12 => self.seq2,
                13..=16 => self.seq1,
                _ => panic!("Invalid sequence number")
            };
            let mut adc_sqr_val = this_sqr.read_volatile();
            let shift = (seq - 1) * 5;
            adc_sqr_val &= !(0b11111 << shift); // Clear the bits
            adc_sqr_val |= (channel) << shift; // Set the bits
            this_sqr.write_volatile(adc_sqr_val);
        }
    }

    pub fn dr_data(&self) -> u16 {
        unsafe {
            self.dr.read_volatile() as u16
        }
    }

    pub fn sr_eoc(&self) -> bool {
        unsafe {
            self.sr.read_volatile() & (0b1 << 1) == 1
        }
    }







}
