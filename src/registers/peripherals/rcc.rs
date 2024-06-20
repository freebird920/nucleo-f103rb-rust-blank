// src/registers/peripherals/rcc.rs

const RCC_BASE: u32 = 0x4002_1000;

pub struct Rcc {
    pub cr: *mut u32,
    pub cfgr: *mut u32,
    pub cir: *mut u32,
    pub apb2rstr: *mut u32,
    pub apb1rstr: *mut u32,
    pub ahbenr: *mut u32,
    pub apb2enr: *mut u32,
    pub apb1enr: *mut u32,
    pub bdcr: *mut u32,
    pub csr: *mut u32,
}

impl Rcc {
    pub fn new() -> Rcc {
        Rcc {
            cr: (RCC_BASE + 0x00) as *mut u32,
            cfgr: (RCC_BASE + 0x04) as *mut u32,
            cir: (RCC_BASE + 0x08) as *mut u32,
            apb2rstr: (RCC_BASE + 0x0C) as *mut u32,
            apb1rstr: (RCC_BASE + 0x10) as *mut u32,
            ahbenr: (RCC_BASE + 0x14) as *mut u32,
            apb2enr: (RCC_BASE + 0x18) as *mut u32,
            apb1enr: (RCC_BASE + 0x1C) as *mut u32,
            bdcr: (RCC_BASE + 0x20) as *mut u32,
            csr: (RCC_BASE + 0x24) as *mut u32,
        }
    }
}
