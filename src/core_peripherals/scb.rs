const BASE_SCB: u32 = 0xE000_ED00;

pub struct Scb {
    icsr: *mut u32,
}

impl Scb {
    pub fn new() -> Scb {
        Scb {
            icsr: (BASE_SCB + 0x04) as *mut u32,
        }
    }

    pub fn icsr_pendsvset_write(&self, ) {
        unsafe {
            self.icsr.write_volatile(1 << 28);
        }
    }
    pub fn icsr_pendsvset_read(&self, ) -> u32 {
        unsafe {
            self.icsr.read_volatile()
        }
    }
}