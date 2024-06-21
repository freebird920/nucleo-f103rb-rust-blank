use cortex_m::asm::delay;

pub struct Delay {
    sys_clock: u32,
}

impl Delay {
    pub fn new(sys_clock: u32) -> Delay {
        Delay {
            sys_clock: sys_clock,
        }
    }

    /// # delay_ms
    /// Delay in milliseconds
    pub fn delay_ms(&self, ms: u32) {
        let cycles = self.sys_clock / (1000 * ms);
        delay(cycles)
    }
    pub fn delay_us(&self, us: u32) {
        let cycles = self.sys_clock / (1000 * 1000 * us);
        delay(cycles)
    }
}
