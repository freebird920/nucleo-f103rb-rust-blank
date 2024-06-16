#[allow(non_snake_case)]


use crate::peripherals::flash::Flash;

const RCC_BASE: u32 = 0x4002_1000;
pub struct Rcc {
    // base: u32,
    cr: *mut u32,
    cfgr: *mut u32,
    apb2enr: *mut u32,
    #[allow(unused)]
    apb1enr: *mut u32,
    // ahbenr: *mut u32,
    // bdcr: *mut u32,
    // csr: *mut u32,
}

impl Rcc {
    pub fn new() -> Rcc {
        // let base_addr = RCC_BASE;
        Rcc {
            // base: RCC_BASE,
            cr: (RCC_BASE + 0x00) as *mut u32,
            cfgr: (RCC_BASE + 0x04) as *mut u32,
            apb2enr: (RCC_BASE + 0x18) as *mut u32,
            apb1enr: (RCC_BASE + 0x1C) as *mut u32,
            // ahbenr: (RCC_BASE + 0x14) as *mut u32,
            // bdcr: (RCC_BASE + 0x20) as *mut u32,
            // csr: (RCC_BASE + 0x24) as *mut u32,
        }
    }

    #[allow(unused)]
    pub fn apb2enr_read(&self) -> u32 {
        unsafe { self.apb2enr.read_volatile() }
    }


    /// ## CR_HSION -  HSI ON
    /// HSI oscillator enabled
    pub fn cr_hsion(&self) {
        unsafe {
            let mut rcc_cr_val = self.cr.read_volatile();
            rcc_cr_val |= (1 << 0); // Enable HSI
            self.cr.write_volatile(rcc_cr_val);
            while self.cr.read_volatile() & (1 << 1) == 0 {} // Wait until HSI is ready
        }
    }

    #[allow(unused)]
    pub fn read_cr_pllrdy(&self) -> bool {
        unsafe {
            let rcc_cr_val = self.cr.read_volatile();
            rcc_cr_val & (1 << 25) != 0
        }
    }

    pub fn read_cfgr(&self) -> u32 {
        unsafe { self.cfgr.read_volatile() }
    }

    #[allow(unused)]
    pub fn read_cr(&self) -> u32 {
        unsafe { self.cr.read_volatile() }
    }

    /// ### set_sys_clock - Set system clock
    /// **IMPORTANT** PLL output freq muts not exceed 72MHz
    /// - 0b0000: PLL input clock x 2 = 8 MHz
    /// - 0b0001: PLL input clock x 3 = 12 MHz
    /// - 0b0010: PLL input clock x 4 = 16 MHz
    /// - 0b0011: PLL input clock x 5 = 20 MHz
    /// - 0b0100: PLL input clock x 6 = 24 MHz
    /// - 0b0101: PLL input clock x 7 = 28 MHz
    /// - 0b0110: PLL input clock x 8 = 32 MHz
    /// - 0b0111: PLL input clock x 9 = 36 MHz
    /// - 0b1000: PLL input clock x 10 = 40 MHz
    /// - 0b1001: PLL input clock x 11 = 44 MHz
    /// - 0b1010: PLL input clock x 12 = 48 MHz
    /// - 0b1011: PLL input clock x 13 = 52 MHz
    /// - 0b1100: PLL input clock x 14 = 56 MHz
    /// - 0b1101: PLL input clock x 15 = 60 MHz
    /// - 0b1110: PLL input clock x 16 = 64 MHz
    /// - 0b1111: PLL input clock x 16 = 64 MHz
    pub fn set_sys_clock(&self, pllmul: u32) {
        // Check if multiplication factor is less than 16
        if (pllmul > 17) {
            panic!("Multiplication factor must be less than 16")
        };
        unsafe {
            // Ensure HSI
            self.cr_hsion();

            // set flash latency
            let flash = Flash::new();
            let flash_latency: u8 = match pllmul {
                0..=4 => 0b0000,
                5..=10 => 0b0001,
                11 => 0b0010,
                _ => 0b0010,
            };
            flash.acr_latency(flash_latency); // Set flash latency
            flash.acr_prftbe(true); // Enable prefetch buffer

            // rcc_cfgr_val &= !(0b1 << 24); // Clear PLLON bit
            self.pllmul_set(pllmul);

            let mut rcc_cfgr_val = self.cfgr.read_volatile();
            rcc_cfgr_val &= !(1 << 16); // Clear PLLSRC bit (select HSI/2)
            rcc_cfgr_val &= !(0b11 << 0); // Clear SW bits
            rcc_cfgr_val |= (0b10 << 0); // Set SW to 0b10 (PLL selected as system clock)
            self.cfgr.write_volatile(rcc_cfgr_val);

            self.cr_pllon();
            // while (self.cfgr.read_volatile() & (0b11 << 2)) != (0b10 << 2) {} // Wait until SWS is PLL
        }
    }

    /// ### CFGR_PLLMUL - PLL multiplication factor
    /// **IMPORTANT** PLL output freq muts not exceed 72MHz
    /// - 0000: PLL input clock x 2
    /// - 0001: PLL input clock x 3
    /// - 0010: PLL input clock x 4
    /// - 0011: PLL input clock x 5
    /// - 0100: PLL input clock x 6
    /// - 0101: PLL input clock x 7
    /// - 0110: PLL input clock x 8
    /// - 0111: PLL input clock x 9
    /// - 1000: PLL input clock x 10
    /// - 1001: PLL input clock x 11
    /// - 1010: PLL input clock x 12
    /// - 1011: PLL input clock x 13
    /// - 1100: PLL input clock x 14
    /// - 1101: PLL input clock x 15
    /// - 1110: PLL input clock x 16
    /// - 1111: PLL input clock x 16
    pub fn pllmul_set(&self, multiplication_factor: u32) {
        if (multiplication_factor > 17) {
            panic!("Multiplication factor must be less than 16")
        };
        unsafe {
            let mut rcc_cfgr_val = self.cfgr.read_volatile();
            rcc_cfgr_val &= !(0b1111 << 18); // Clear PLLMUL bits
            rcc_cfgr_val |= (multiplication_factor << 18); // Set PLLMUL bits
            self.cfgr.write_volatile(rcc_cfgr_val);
        }
    }


    #[allow(unused)]
    /// ## CFGR_ADCPRE - ADC prescaler
    /// #### @param **adcpre**
    /// **IMPORTANT** PLCK2 / ADCPRE > 14MHz <br/>
    /// - 00: PCLK2 divided by 2 <br/>
    /// - 01: PCLK2 divided by 4 <br/>
    /// - 10: PCLK2 divided by 6 <br/>
    /// - 11: PCLK2 divided by 8 <br/>
    pub fn cfgr_adcpre(&self, adcpre: u32) {
        unsafe {
            let mut rcc_cfgr_val = self.cfgr.read_volatile();
            rcc_cfgr_val &= !(0b11 << 14); // Clear ADCPRE bits
            rcc_cfgr_val |= (adcpre << 14); // Set ADCPRE bits
            self.cfgr.write_volatile(rcc_cfgr_val);
        }
    }

    fn cr_pllon(&self) {
        unsafe {
            let mut rcc_cr_val = self.cr.read_volatile();
            rcc_cr_val |= (1 << 24); // Enable PLL
            self.cr.write_volatile(rcc_cr_val);
            while self.cr.read_volatile() & (1 << 25) == 0 {} // Wait until PLL is ready
            while (self.cfgr.read_volatile() & (0b11 << 2)) != (0b10 << 2) {} // Wait until SWS is PLL
        }
    }

    // pub fn APB2ENR(&self) -> *mut u32 {
    //     (self.base + 0x18) as *mut u32
    // }

    #[allow(unused)]
    pub fn enable_adc1(&self) {
        unsafe {
            let mut apb2enr_val = self.apb2enr.read_volatile();
            apb2enr_val |= (1 << 9); // ADC1 클럭 활성화
            self.apb2enr.write_volatile(apb2enr_val);
        }
    }

    pub fn APB2ENR_ADC1EN(&self, enable: bool) {
        unsafe {
            let mut apb2enr_val = self.apb2enr.read_volatile();
            if enable {
                apb2enr_val |= (1 << 9); // Enable ADC1
            } else {
                apb2enr_val &= !(1 << 9); // Disable ADC1
            }
            self.apb2enr.write_volatile(apb2enr_val);
        }
    }
    pub fn abp2enr_afioen(&self, enable: bool) {
        unsafe {
            let mut apb2enr_val = self.apb2enr.read_volatile();
            if enable {
                apb2enr_val |= (1 << 0); // Enable AFIO
            } else {
                apb2enr_val &= !(1 << 0); // Disable AFIO
            }
            self.apb2enr.write_volatile(apb2enr_val);
        }
    }

    /// ## APB2ENR_IOPx_EN - I/O port x clock enable
    /// #### @param **iop_x_en**
    /// - IOPAEN = 2
    /// - IOPBEN = 3
    /// - IOPCEN = 4
    /// - IOPDEN = 5
    /// - IOPEEN = 6
    /// #### @param **enable**
    /// - true: Enable IOPx
    /// - false: Disable IOPx
    pub fn apb2enr_iop_en(&self, iop_x: u8, enable: bool) -> Result<(), &'static str> {
        if !(0..=5).contains(&iop_x) {
            return Err("Invalid I/O port");
        }

        unsafe {
            let mut apb2enr_val = self.apb2enr.read_volatile();
            let bit = (iop_x + 2) as u8;
            if enable {
                apb2enr_val |= 0b1 << bit; // Enable IOPx
            } else {
                apb2enr_val &= !(0b1 << bit); // Disable IOPx
            }
            self.apb2enr.write_volatile(apb2enr_val);
        }

        Ok(())
    }

    pub fn APB1ENR_I2C1EN(&self, enable: bool) {
        unsafe {
            let mut apb1enr_val = self.apb1enr.read_volatile();
            if enable {
                apb1enr_val |= (1 << 21); // Enable I2C1
            } else {
                apb1enr_val &= !(1 << 21); // Disable I2C1
            }
            self.apb1enr.write_volatile(apb1enr_val);
        }
    }
    pub fn ABP1ENR_I2C2EN(&self, enable: bool) {
        unsafe {
            let mut apb1enr_val = self.apb1enr.read_volatile();
            if enable {
                apb1enr_val |= (1 << 22); // Enable I2C2
            } else {
                apb1enr_val &= !(1 << 22); // Disable I2C2
            }
            self.apb1enr.write_volatile(apb1enr_val);
        }
    }
    pub fn apb1enr_tim_gp_en(&self, tim_gp_x: u8, enable: bool) -> Result<(), &str> {
        match tim_gp_x {
            2..=5 => {
                unsafe {
                    let mut apb1enr_val = self.apb1enr.read_volatile();
                    let bit = (tim_gp_x as u32) - 2;
                    if enable {
                        apb1enr_val |= 1 << bit; // Enable TIMx
                    } else {
                        apb1enr_val &= !(1 << bit); // Disable TIMx
                    }
                    self.apb1enr.write_volatile(apb1enr_val);
                }
                Ok(())
            }
            _ => Err("Invalid general purpose timer"),
        }
    }

    pub fn APB1ENR_TIM2EN(&self, enable: bool) {
        unsafe {
            let mut apb1enr_val = self.apb1enr.read_volatile();
            if enable {
                apb1enr_val |= (1 << 0); // Enable TIM2
            } else {
                apb1enr_val &= !(1 << 0); // Disable TIM2
            }
            self.apb1enr.write_volatile(apb1enr_val);
        }
    }

    pub fn get_sys_clock(&self) -> u32 {
        // RCC 레지스터 하드코딩된 값
        let rcc_cfgr = self.read_cfgr(); // 예시 값, 실제 값으로 교체 필요

        // HSI 클럭 속도
        let hsi_clk = 8_000_000; // 8 MHz
                                 // HSE 클럭 속도 (기본값)

        // SWS 비트 확인 (시스템 클럭 소스)
        let sws = (rcc_cfgr >> 2) & 0b11;

        let sysclk = match sws {
            0b00 => {
                // HSI 사용
                hsi_clk
            }
            0b01 => {
                // HSE 사용
                panic!("HSE not supported")
            }
            0b10 => {
                // PLL 사용
                // PLL 소스 확인 (PLLSRC 비트)
                let pllsrc = (rcc_cfgr >> 16) & 0b1;
                let pll_clk_in = if pllsrc == 0 {
                    // HSI/2
                    hsi_clk / 2
                } else {
                    0
                    // HSE
                    // if (rcc_cfgr >> 17) & 0b1 == 1 {
                    //     hse_clk / 2
                    // } else {
                    //     hse_clk
                    // }
                };

                // PLL 곱셈 계수 확인 (PLLMUL 비트)
                let pllmul = ((rcc_cfgr >> 18) & 0b1111) + 2;
                pll_clk_in * pllmul
            }
            _ => {
                // 예약된 값 (사용되지 않음)
                0
            }
        };

        // AHB 프리스케일러 확인 (HPRE 비트)
        let hpre = (rcc_cfgr >> 4) & 0b1111;
        let ahb_prescaler = match hpre {
            0b0000 => 1,
            0b1000 => 2,
            0b1001 => 4,
            0b1010 => 8,
            0b1011 => 16,
            0b1100 => 64,
            0b1101 => 128,
            0b1110 => 256,
            0b1111 => 512,
            _ => 1,
        };

        sysclk / ahb_prescaler
    }
}
