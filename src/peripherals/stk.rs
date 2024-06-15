// SysTic

const BASE_ADDRESS_SYS_TICK: u32 = 0x0E000_E010;
pub struct Stk {
    ctrl: *mut u32,
    load: *mut u32,
    val: *mut u32,
    calib: *mut u32,
}

impl Stk {
    pub fn new() -> Stk {
        Stk {
            ctrl: (BASE_ADDRESS_SYS_TICK + 0x00) as *mut u32,
            load: (BASE_ADDRESS_SYS_TICK + 0x04) as *mut u32,
            val: (BASE_ADDRESS_SYS_TICK + 0x08) as *mut u32,
            calib: (BASE_ADDRESS_SYS_TICK + 0x0C) as *mut u32,
        }
    }

    pub fn ctrl_read(&self) -> u32 {
        unsafe { self.ctrl.read_volatile() }
    }
    pub fn ctrl_enable(&self, enable: bool) {
        unsafe {
            let mut ctrl_val = self.ctrl.read_volatile();
            if enable {
                ctrl_val |= 0b1;
            } else {
                ctrl_val &= !0b1;
            }
            self.ctrl.write_volatile(ctrl_val);
        }
    }

    pub fn calib_read(&self) -> u32 {
        unsafe { self.calib.read_volatile() }
    } 
    pub fn calib_noref_read(&self) -> u32 {
        unsafe { (self.calib.read_volatile()>> 31) & 0b1 }
    }
    pub fn calib_skew_read(&self) -> u32 {
        unsafe { (self.calib.read_volatile() >> 30) & 0b1 }
    }
    pub fn calib_tenms_read(&self) -> u32 {
        unsafe { self.calib.read_volatile() & 0b1111_1111_1111_1111_1111 }
    }
}
