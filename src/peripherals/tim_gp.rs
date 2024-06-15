use crate::peripherals::rcc::Rcc;

pub struct TimGp {
    tim_x: u8,
    cr1: *mut u32,
}

impl TimGp {
    /// ## Create a new instance of TimGp
    /// **!Important** : TimGp 를 사용하기 전에 먼저 RCC_APB1의 TIMxENR 레지스터를 설정해야 합니다. <br/>
    /// **@param gp_tim_x** x, x = TIMx( 1< x < 6)
    pub fn new(gp_tim_x: u8) -> TimGp {
        let base = match gp_tim_x {
            2 => 0x4000_0000,
            3 => 0x4000_0400,
            4 => 0x4000_0800,
            5 => 0x4000_0C00,
            _ => panic!("Invalid general purpose timer"),
        };

        TimGp {
            tim_x: gp_tim_x,
            cr1: (base + 0x00) as *mut u32,
        }
    }

    pub fn tim_gp_clock_enable(&self) {
        let rcc = Rcc::new();
        rcc.apb1enr_tim_gp_en(self.tim_x, true);
    }


}
