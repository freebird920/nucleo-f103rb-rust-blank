use crate::registers::peripherals::rcc::Rcc;

pub struct UseRcc<'life_use_rcc> {
    rcc: &'life_use_rcc Rcc,
}

impl<'life_use_rcc> UseRcc<'life_use_rcc> {
    pub fn new(rcc: &'life_use_rcc Rcc) -> UseRcc<'life_use_rcc> {
        UseRcc { rcc: rcc }
    }

    /// ## apb2enr_iop_x_en_set
    /// iop_x_en bit set in APB2ENR register <br/>s
    /// ### @params
    /// - iop_x: u8 (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H)
    /// - val: u32 (0: disable, 1: enable)
    pub fn abp2enr_iop_x_en_set(&self, iop_x: u8, val: u32) -> Result<(), &'static str> {
        self.rcc.apb2enr_iop_x_en_set(iop_x, val)
    }

    
}
