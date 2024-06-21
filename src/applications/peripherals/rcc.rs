use crate::registers::peripherals::flash::Flash;
use crate::registers::peripherals::rcc::Rcc;

pub struct UseRcc<'life_use_rcc> {
    rcc: &'life_use_rcc Rcc,
}

impl<'life_use_rcc> UseRcc<'life_use_rcc> {
    pub fn new(rcc: &'life_use_rcc Rcc) -> UseRcc<'life_use_rcc> {
        UseRcc { rcc: rcc }
    }
}

impl<'life_use_rcc> UseRcc<'life_use_rcc> {
    /// # apb2enr_iop_x_en_set
    /// iop_x_en bit set in APB2ENR register <br/>s
    /// ## Parameters
    /// - iop_x: u8 (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H)
    /// - val: u32 (0: disable, 1: enable)
    pub fn abp2enr_iop_x_en_set(&self, iop_x: u8, val: u32) -> Result<(), &'static str> {
        self.rcc.apb2enr_iop_x_en_set(iop_x, val)
    }
}

impl<'life_use_rcc> UseRcc<'life_use_rcc> {
    /// # set pll
    /// ## Parameters
    /// ### @param multiplication_factor
    /// - 0000: PLL input clock x 2 (default)
    /// - 0001: PLL input clock x 3 (4 * 3 = 12 MHz)
    /// - 0010: PLL input clock x 4 (4 * 4 = 16 MHz)
    /// - 0011: PLL input clock x 5 (4 * 5 = 20 MHz)
    /// - 0100: PLL input clock x 6 (4 * 6 = 24 MHz)
    /// - 0101: PLL input clock x 7 (4 * 7 = 28 MHz)
    /// - 0110: PLL input clock x 8 (4 * 8 = 32 MHz)
    /// - 0111: PLL input clock x 9 (4 * 9 = 36 MHz)
    /// - 1000: PLL input clock x 10 (4 * 10 = 40 MHz)
    /// - 1001: PLL input clock x 11 (4 * 11 = 44 MHz)
    /// - 1010: PLL input clock x 12 (4 * 12 = 48 MHz)
    /// - 1011: PLL input clock x 13 (4 * 13 = 52 MHz)
    /// - 1100: PLL input clock x 14 (4 * 14 = 56 MHz)
    /// - 1101: PLL input clock x 15 (4 * 15 = 60 MHz)
    /// - 1110: PLL input clock x 16 (4 * 16 = 64 MHz)
    /// - 1111: PLL input clock x 16 (4 * 16 = 64 MHz)
    pub fn pll_set(&self, multiplication_factor: u32) -> Result<(), &'static str> {
        let flash_latency = match multiplication_factor {
            0b0000 | 0b0001 | 0b0010 | 0b0011 | 0b0100 => 0,
            0b0101 | 0b0110 | 0b0111 | 0b1000 | 0b1001 | 0b1010 => 1,
            0b1011 | 0b1100 | 0b1101 | 0b1110 | 0b1111 => 2,
            _ => return Err("invalid multiplication_factor"),
        };

        // # 1 HSI ON
        self.rcc.cr_hsion(); // SET HSI ON, Wait until HSI is ready.

        // # 2 FLASH LATENCY SET
        let flash = Flash::new();
        flash.acr_latency(flash_latency); // Set flash latency
        flash.acr_prftbe(true); // Enable prefetch buffer

        // # 3 PLL ON
        // rcc_cfgr_val &= !(0b1 << 24); // Clear PLLON bit
        self.rcc.cfgr_pllmul_set(multiplication_factor)?; // Set PLLMUL bits 
        self.rcc.cfgr_pllsrc_set(0)?; // Set PLLSRC bit (HSI/2)
        self.rcc.cfgr_sw_set(0b10)?; // Set SW bits (PLL selected as system clock)

        // # 4 PLL ON
        self.rcc.cr_pllon();
        Ok(())
    }
}
