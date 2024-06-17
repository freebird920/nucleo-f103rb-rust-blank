const BASE_NVIC: u32 = 0xE000_E100;

pub struct Nvic {
    
    iser_x: *mut u32,
}

impl Nvic {
    pub fn new(interrupt_no: u32) -> Result<Nvic, &'static str> {
        match interrupt_no {
            0..=31 => (),
            32..=63 => (),
            64..=67 => (),
            _ => Err("Invalid interrupt_no")?,
        }

        Ok(Nvic {
            iser_x: (BASE_NVIC + (interrupt_no / 32) * 4) as *mut u32,
        })
    }

   pub fn iser_set(&self, interrupt_no: u32, enable: bool) {
        unsafe {
            let mut iser_val = self.iser_x.read_volatile();
            if enable {
                iser_val |= 1 << (interrupt_no % 32);
            } else {
                iser_val &= !(1 << (interrupt_no % 32));
            }
            self.iser_x.write_volatile(iser_val);
        }
    }
}
