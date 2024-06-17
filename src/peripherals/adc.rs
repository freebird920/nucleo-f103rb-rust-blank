pub struct Adc {
    base: u32, // Storing the base address directly as u32
    sr: *mut u32,
    cr1: *mut u32,
    cr2: *mut u32,
    seq1: *mut u32,
    seq2: *mut u32,
    seq3: *mut u32,
    dr: *mut u32,
    smpr1: *mut u32,
    smpr2: *mut u32,
}

impl Adc {
    /// ### Adc::new
    /// **adc_x** 1 | 2
    pub fn new(adc_x: u8) -> Result<Adc, &'static str> {
        let base_addr: u32 = match adc_x {
            1 => 0x4001_2800,
            2 => 0x4001_2400,
            _ => return Err("Invalid ADC number"),
        };

        Ok(Adc {
            base: base_addr,
            sr: (base_addr + 0x00) as *mut u32,
            cr1: (base_addr + 0x04) as *mut u32,
            cr2: (base_addr + 0x08) as *mut u32,
            seq1: (base_addr + 0x2C) as *mut u32,
            seq2: (base_addr + 0x30) as *mut u32,
            seq3: (base_addr + 0x34) as *mut u32,
            dr: (base_addr + 0x4C) as *mut u32,
            smpr1: (base_addr + 0x0C) as *mut u32,
            smpr2: (base_addr + 0x10) as *mut u32,
        })
    }
    pub fn base_read(&self) -> u32 {
        self.base
    }
    pub fn cr2_read(&self) -> u32 {
        unsafe { self.cr2.read_volatile() }
    }
    pub fn cr2_addr(&self) -> u32 {
        self.cr2 as u32
    }
    /// ### CR1_ADON -  A/D Converter ON / OFF
    /// - ADC 제어 레지스터에서 ADON 비트 이외의 다른 비트가 동시에 변경되면 변환이 트리거되지 않음. <br/>
    /// - 만약 ADON 비트 이외의 다른 비트를 변경할 때 ADON 비트를 함께 변경하려고 하면, 변환이 트리거되지 않습니다. <br/>
    /// - ADC 설정 시 다른 비트와 ADON 비트를 동시에 변경하지 않도록 주의해야 합니다. 먼저 다른 비트를 설정한 후, ADON 비트를 변경하여 ADC를 켜거나 끄는 것이 좋습니다. <br/>
    pub fn cr2_adon(&self, enable: bool) {
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
    // ## cr2_cont
    /// ### CR2_CONT - Continuous Conversion
    /// - ADC 제어 레지스터에서 CONT 비트는 연속 변환 모드와 단일 변환 모드를 선택합니다. <br/>
    pub fn cr2_cont(&self, enable: bool) {
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

    /// ### CR2_CAL - Calibration
    /// - ADC 제어 레지스터에서 CAL 비트는 ADC의 자체 보정을 시작하거나 보정 중인지를 나타냅니다. <br/>
    /// - CAL 비트는 ADCAL 비트가 0으로 설정되면 자동으로 0으로 클리어됩니다. <br/>
    pub fn cr2_cal(&self) {
        unsafe {
            let mut adc_cr2_val = self.cr2.read_volatile();
            adc_cr2_val |= (1 << 2); // Start calibration
            self.cr2.write_volatile(adc_cr2_val);
            while (self.cr2.read_volatile() & (1 << 2)) != 0 {
                // Wait for calibration to complete
            }
        }
    }
    /// ### SMPR_SMP - Sampling Time Selection
    /// - ADC 채널별 샘플링 시간을 설정합니다. <br/>
    /// smp 값은 0~7까지 설정할 수 있습니다. <br/>
    /// - 0b000 : 1.5 cycles
    /// - 0b001 : 7.5 cycles
    /// - 0b010 : 13.5 cycles
    /// - 0b011 : 28.5 cycles
    /// - 0b100 : 41.5 cycles
    /// - 0b101 : 55.5 cycles
    /// - 0b110 : 71.5 cycles
    /// - 0b111 : 239.5 cycles
    /// - 샘플링 시간이 길수록 노이즈가 줄어들지만, 변환 속도가 느려집니다. <br/>
    pub fn smpr_smp(&self, channel: u32, smp: u32) -> Result<(), &'static str> {
        if smp > 7 {
            return Err("Invalid SMP value");
        }
        let smpr = match channel {
            0..=9 => self.smpr2,
            10..=17 => self.smpr1,
            _ => return Err("Invalid channel number"),
        };
        let offset = channel % 10;
        let shift = offset * 3;
        unsafe {
            let mut smpr_val = smpr.read_volatile();
            smpr_val &= !(0b111 << shift); // Clear the bits
            smpr_val |= (smp << shift); // Set the bits
            smpr.write_volatile(smpr_val);
        }
        Ok(())
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
    pub fn cr2_extsel(&self, extsel: u32) {
        unsafe {
            let mut adc_cr2_val = self.cr2.read_volatile();
            adc_cr2_val &= !(0b111 << 17); // Clear the bits
            adc_cr2_val |= (extsel) << 17; // Set the bits
            self.cr2.write_volatile(adc_cr2_val);
        }
    }
    pub fn cr2_swstart(&self, enable: bool) {
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
    pub fn sr_eoc_read(&self) -> bool {
        unsafe { self.sr.read_volatile() & (0b1 << 1) != 0 }
    }

pub fn sqr3_sq(&self, seq: u8, channel: u32) {
    unsafe {
        let mut adc_sqr3_val = self.seq3.read_volatile();
        let shift = (seq - 1) * 5;
        adc_sqr3_val &= !(0b11111 << shift); // Clear the bits
        adc_sqr3_val |= (channel & 0b11111) << shift; // Set the bits
        self.seq3.write_volatile(adc_sqr3_val);
    }
}
    pub fn sqr_read(&self, seq: u8) -> u32 {
        unsafe {
            match seq {
                1 => self.seq1.read_volatile(),
                2 => self.seq2.read_volatile(),
                3 => self.seq3.read_volatile(),
                _ => 0,
            }
        }
    }
    pub fn sqr_sq(&self, seq: u8, channel: u32) {
        unsafe {
            let this_sqr = match seq {
                1..=6 => self.seq3,
                7..=12 => self.seq2,
                13..=16 => self.seq1,
                _ => panic!("Invalid sequence number"),
            };
            let mut adc_sqr_val = this_sqr.read_volatile();
            let shift = (seq - 1) * 5;
            adc_sqr_val &= !(0b11111 << shift); // Clear the bits
            adc_sqr_val |= (channel) << shift; // Set the bits
            this_sqr.write_volatile(adc_sqr_val);
        }
    }

    pub fn dr_data(&self) -> u32 {
        unsafe { self.dr.read_volatile() as u32 }
    }

    pub fn sr_eoc(&self) -> bool {
        unsafe { self.sr.read_volatile() & (0b1 << 1) == 1 }
    }
}
