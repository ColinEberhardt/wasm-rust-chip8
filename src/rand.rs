// https://codereview.stackexchange.com/questions/169172/complementary-multiply-with-carry-in-rust/169338
pub const CMWC_CYCLE: usize = 4096;
const PHI: u32 = 0x9e3779b9;

pub struct ComplementaryMultiplyWithCarryGen {
    pub q: [u32; CMWC_CYCLE],
    pub c: u32,
    pub i: usize,
}

impl ComplementaryMultiplyWithCarryGen {
    pub fn new(seed: u32) -> ComplementaryMultiplyWithCarryGen {
        let mut q = [0; CMWC_CYCLE];

        q[0] = seed;
        q[1] = seed.wrapping_add(PHI);
        q[2] = seed.wrapping_add(PHI).wrapping_add(PHI);

        for i in 3..CMWC_CYCLE {
            let window = &mut q[i - 3..i + 1];
            window[3] = window[0] ^ window[1] ^ PHI ^ seed;
        }

        ComplementaryMultiplyWithCarryGen {
            q: q,
            c: 362436,
            i: 4095,
        }
    }

    pub fn random(&mut self) -> u32 {
        const A: u64 = 18782;
        const R: u32 = 0xfffffffe;

        self.i = (self.i + 1) & (CMWC_CYCLE - 1);
        let t = A * self.q[self.i] as u64 + self.c as u64;

        self.c = (t >> 32) as u32;
        let mut x = (t + self.c as u64) as u32;
        if x < self.c {
            x += 1;
            self.c += 1;
        }

        self.q[self.i] = R - x;
        self.q[self.i]
    }
}