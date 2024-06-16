use crate::peripherals::rcc::Rcc;

pub struct TimGp {
    tim_x: u8,
    // cr1: *mut u32,
}

impl TimGp {
    /// ## Create a new instance of TimGp
    /// **!Important** : TimGp 를 사용하기 전에 먼저 RCC_APB1의 TIMxENR 레지스터를 설정해야 합니다. <br/>
    /// **@param gp_tim_x** x, x = TIMx( 1< x < 6)
    pub fn new(gp_tim_x: u8) -> Result<TimGp,&'static str> {
        let base: Result<u32, &str> = match gp_tim_x {
            2 => Ok(0x4000_0000),
            3 => Ok(0x4000_0400),
            4 => Ok(0x4000_0800),
            5 => Ok(0x4000_0C00),
            _ => Err("Invalid general purpose timer"),
        };
        let base_value = match base {
            Ok(value) => value,
            Err(e) => {
                return Err(e);
            }
        };


        
        Ok(
        TimGp {
            tim_x: gp_tim_x,
            // cr1: (base_value + 0x00) as *mut u32,
        })
    }

    pub fn tim_gp_clock_enable(&self) {
        let rcc = Rcc::new();
        let _  = rcc.apb1enr_tim_gp_en(self.tim_x, true);
    }


}
