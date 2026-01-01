use std::fs::File;
use std::io::Read;

pub struct Rng {
    state: u64,
    inc: u64,
}

impl Rng {
    pub fn new() -> Self {
        let mut seed = [0u8; 8];
        if let Ok(mut f) = File::open("/dev/urandom") {
            let _ = f.read_exact(&mut seed);
        }
        let state = u64::from_ne_bytes(seed);
        Self {
            state,
            inc: (state >> 31) | 1,
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc);
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;
        (xorshifted >> rot) | (xorshifted << ((!rot).wrapping_add(1) & 31))
    }

    pub fn gen_range(&mut self, upper: u32) -> u32 {
        if upper == 0 {
            return 0;
        }
        self.next_u32() % upper
    }
}
