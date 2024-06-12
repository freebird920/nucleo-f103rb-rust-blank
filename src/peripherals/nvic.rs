#![allow(non_snake_case)]
struct NVIC {
    base: u32,
}
const NVIC_BASE: u32 = 0xE000_E100;

const NVIC_ISER: [*mut u32; 3] = [
    (NVIC_BASE + 0x000) as *mut u32, // ISER[0]
    (NVIC_BASE + 0x004) as *mut u32, // ISER[1]
    (NVIC_BASE + 0x008) as *mut u32, // ISER[2]
];

const OFFSET_NVIC_ICERx: [*mut u32; 3] = [
    (NVIC_BASE + 0x080) as *mut u32, // ICER[0]
    (NVIC_BASE + 0x084) as *mut u32, // ICER[1]
    (NVIC_BASE + 0x088) as *mut u32, // ICER[2]
];

const OFFSET_NVIC_ISPRx: [*mut u32; 3] = [
    (NVIC_BASE + 0x100) as *mut u32, // ISPR[0]
    (NVIC_BASE + 0x104) as *mut u32, // ISPR[1]
    (NVIC_BASE + 0x108) as *mut u32, // ISPR[2]
];

const OFFSET_NVIC_ICPRx: [*mut u32; 3] = [
    (NVIC_BASE + 0x180) as *mut u32, // ICPR[0]
    (NVIC_BASE + 0x184) as *mut u32, // ICPR[1]
    (NVIC_BASE + 0x188) as *mut u32, // ICPR[2]
];

const OFFSET_NVIC_IABRx: [*mut u32; 3] = [
    (NVIC_BASE + 0x200) as *mut u32, // IABR[0]
    (NVIC_BASE + 0x204) as *mut u32, // IABR[1]
    (NVIC_BASE + 0x208) as *mut u32, // IABR[2]
];

impl NVIC {
    pub fn interrupt_group(&self, position: u8) -> Result<u8, &'static str> {
        match position {
            0..=31 => Ok(0),  // ISER[0], ICER[0], ISPR[0], ICPR[0], IABR[0]
            32..=63 => Ok(1), // ISER[1], ICER[1], ISPR[1], ICPR[1], IABR[1]
            64..=67 => Ok(2), // ISER[2], ICER[2], ISPR[2], ICPR[2], IABR[2]
            _ => Err("Invalid interrupt position"),
        }
    }

    pub fn ISER(&self, position: u8) -> Result<*mut u32, &'static str> {
        let group = self.interrupt_group(position).unwrap();
        Ok(NVIC_ISER[group as usize] as *mut u32)
    }


}