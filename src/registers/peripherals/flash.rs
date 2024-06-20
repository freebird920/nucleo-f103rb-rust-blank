pub struct Flash{
    acr: *mut u32,
}

impl Flash{
    pub fn new() -> Flash{
        let base_address = 0x4002_2000;
        Flash{
            acr: (base_address + 0x00) as *mut u32,
        }
    }
}

impl Flash { // acr()
    pub fn acr_read(&self) -> u32 {
        unsafe { self.acr.read_volatile() }
    } 

}